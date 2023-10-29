#![doc = include_str!("../README.md")]

mod builder;
mod config;
pub mod error;
pub use builder::GatewayBuilder;
pub use config::Config;

use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use error::{ReceiveError, ReceiveErrorKind, SendError, SendErrorKind};
use futures::{future::poll_fn, SinkExt, Stream};
use http::{header::USER_AGENT, HeaderValue};
use rive_models::{
    authentication::Authentication,
    event::{ClientEvent, ServerEvent},
};
use tokio::{net::TcpStream, time};
use tokio_websockets::{Error as WsError, MaybeTlsStream, Message as WsMessage, WebsocketStream};
use tracing::{debug, instrument};

pub type HeartbeatFn = fn() -> i32;

type Socket = WebsocketStream<MaybeTlsStream<TcpStream>>;

/// Base WebSocket API URL of official Revolt instance
pub const BASE_URL: &str = "wss://ws.revolt.chat";

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);

#[derive(Debug)]
enum NextAction {
    Authenticate,
}

/// A wrapper for Revolt WebSocket API
#[derive(Debug)]
pub struct Gateway {
    socket: Option<Socket>,
    config: Config,
    heartbeat_interval: Option<time::Interval>,
    next_action: Option<NextAction>,
}

impl Gateway {
    pub fn new(auth: Authentication) -> Self {
        Self::with_url(BASE_URL, auth)
    }

    pub fn with_url(url: impl Into<String>, auth: Authentication) -> Self {
        Self::with_config(Config {
            auth,
            base_url: url.into(),
            ..Default::default()
        })
    }

    pub fn with_config(config: Config) -> Self {
        Self {
            socket: None,
            config,
            heartbeat_interval: None,
            next_action: None,
        }
    }

    pub fn builder() -> GatewayBuilder {
        GatewayBuilder::new()
    }

    #[instrument(skip(self))]
    pub async fn next_event(&mut self) -> Result<ServerEvent, ReceiveError> {
        #[derive(Debug)]
        enum Action {
            Connect,
            Authenticate,
            Heartbeat,
            Message(Option<Result<WsMessage, WsError>>),
        }

        loop {
            let next_action = |cx: &mut Context<'_>| {
                if self.socket.is_none() {
                    return Poll::Ready(Action::Connect);
                }

                if let Some(action) = self.next_action.take() {
                    match action {
                        NextAction::Authenticate => return Poll::Ready(Action::Authenticate),
                    }
                }

                if self.config.heartbeat.is_some()
                    && self
                        .heartbeat_interval
                        .as_mut()
                        .map_or(false, |interval| interval.poll_tick(cx).is_ready())
                {
                    return Poll::Ready(Action::Heartbeat);
                }

                if let Poll::Ready(message) =
                    Pin::new(self.socket.as_mut().expect("connected")).poll_next(cx)
                {
                    return Poll::Ready(Action::Message(message));
                }

                Poll::Pending
            };

            match poll_fn(next_action).await {
                Action::Connect => {
                    debug!("connecting to the API");
                    self.connect().await?;
                }
                Action::Authenticate => {
                    debug!("sending authenticate event");
                    self.send(&ClientEvent::Authenticate {
                        token: self.config.auth.value(),
                    })
                    .await
                    .map_err(ReceiveError::from_send)?;
                }
                Action::Heartbeat => {
                    if let Some(heartbeat_fn) = self.config.heartbeat {
                        debug!("sending heartbeat event");
                        self.send(&ClientEvent::Ping {
                            data: (heartbeat_fn)(),
                        })
                        .await
                        .map_err(|err| {
                            ReceiveError::new(ReceiveErrorKind::SendMessage, Some(Box::new(err)))
                        })?;
                    }
                }
                Action::Message(Some(Ok(msg))) => {
                    if let Some(text) = msg.as_text() {
                        debug!("received a text message");
                        let event = Self::deserialize_server_event(text).map_err(|err| {
                            ReceiveError::new(ReceiveErrorKind::Deserialize, Some(Box::new(err)))
                        });

                        if matches!(event, Ok(ServerEvent::Authenticated)) {
                            self.heartbeat_interval = Some(time::interval(HEARTBEAT_INTERVAL));
                        }

                        return event;
                    } else if msg.is_close() {
                        debug!("received a close message");
                        self.disconnect();
                    }
                }
                Action::Message(None) => {
                    debug!("API connection closed");
                    self.disconnect();
                    return Err(ReceiveError::new(ReceiveErrorKind::Io, None));
                }
                Action::Message(Some(Err(err))) => {
                    debug!("received an error");
                    return Err(ReceiveError::new(ReceiveErrorKind::Io, Some(Box::new(err))));
                }
            }
        }
    }

    pub async fn send(&mut self, event: &ClientEvent) -> Result<(), SendError> {
        self.socket
            .as_mut()
            .ok_or(SendError::new(SendErrorKind::Send, None))?
            .send(Self::encode_client_event(event)?)
            .await
            .map_err(|source| SendError::new(SendErrorKind::Send, Some(Box::new(source))))
    }

    async fn connect(&mut self) -> Result<(), ReceiveError> {
        let (socket, _) = tokio_websockets::ClientBuilder::from_uri(
            self.config.base_url.clone().try_into().expect("valid url"),
        )
        .add_header(USER_AGENT, HeaderValue::from_static("rive-gateway"))
        .connect()
        .await
        .map_err(|source| ReceiveError::new(ReceiveErrorKind::Reconnect, Some(Box::new(source))))?;

        self.socket = Some(socket);
        if !matches!(self.config.auth, Authentication::None) {
            self.next_action = Some(NextAction::Authenticate);
        }

        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), SendError> {
        let res = self
            .socket
            .as_mut()
            .ok_or(SendError::new(SendErrorKind::Send, None))?
            .send(WsMessage::close(None, ""))
            .await
            .map_err(|source| SendError::new(SendErrorKind::Send, Some(Box::new(source))));

        self.disconnect();

        res
    }

    fn disconnect(&mut self) {
        self.socket = None;
        self.heartbeat_interval = None;
    }

    fn encode_client_event(event: &ClientEvent) -> Result<WsMessage, SendError> {
        serde_json::to_string(event)
            .map(WsMessage::text)
            .map_err(|source| SendError::new(SendErrorKind::Serialize, Some(Box::new(source))))
    }

    fn decode_server_event(value: &str) -> Result<ServerEvent, ReceiveError> {
        serde_json::from_str(value).map_err(|source| {
            ReceiveError::new(ReceiveErrorKind::Deserialize, Some(Box::new(source)))
        })
    }
}
