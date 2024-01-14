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
use tokio_websockets::{Error as WsError, MaybeTlsStream, Message as WsMessage, WebSocketStream};
use tracing::{debug, instrument};

/// Type alias of a raw Websocket object.
type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Base WebSocket API URL of official Revolt instance.
pub const BASE_URL: &str = "wss://ws.revolt.chat";

/// Default heartbeat interval.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);

/// The next action that the client should perform.
#[derive(Debug)]
enum NextAction {
    /// Send authentication packet
    Authenticate,
}

/// Client for Revolt Websocket API.
///
/// Initially the client does not connect to the API. The connection attempt
/// occurs after the first call to [`next_event`]. After that, [`next_event`]
/// must be called repeatedly in order for the client to maintain the connection
/// and update the internal state.
///
/// # Examples
///
/// Print new messages and joined users:
///
/// ```no_run
/// use std::{env, error::Error};
///
/// use tracing::{info, warn};
///
/// use rive_gateway::Gateway;
/// use rive_models::{authentication::Authentication, event::ServerEvent};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let auth = Authentication::BotToken(env::var("TOKEN")?);
/// let mut gateway = Gateway::new(auth);
///
/// loop {
///     match gateway.next_event().await {
///         Ok(event) => match event {
///             ServerEvent::Message(message) => {
///                 info!("New message with content: {:?}", message.content);
///             }
///             ServerEvent::ServerMemberJoin(event) => {
///                 info!(
///                     "User with ID {} joined server with ID {}",
///                     event.user, event.id
///                 )
///             }
///             _ => {}
///         },
///         Err(err) => {
///             warn!(?err, "error receiving event");
///             break;
///         }
///     }
/// }
/// # Ok(()) }
/// ```
///
/// [`next_event`]: Gateway::next_event
#[derive(Debug)]
pub struct Gateway {
    /// Websocket connection.
    socket: Option<Socket>,
    /// User provided configuration.
    config: Config,
    /// Interval of periodic heartbeat sending.
    heartbeat_interval: Option<time::Interval>,
    /// Next action client should perform in response of the Websocket events.
    next_action: Option<NextAction>,
}

impl Gateway {
    /// Create a new [`Gateway`] with given authentication token and default
    /// configuration.
    pub fn new(auth: Authentication) -> Self {
        Self::with_url(BASE_URL, auth)
    }

    /// Create a new [`Gateway`] with given base URL and authentication token.
    pub fn with_url(url: impl Into<String>, auth: Authentication) -> Self {
        Self::with_config(Config {
            auth,
            base_url: url.into(),
            ..Default::default()
        })
    }

    /// Create a new [`Gateway`] with given configuration.
    pub fn with_config(config: Config) -> Self {
        Self {
            socket: None,
            config,
            heartbeat_interval: None,
            next_action: None,
        }
    }

    /// Create a new builder to configure and create a [`Gateway`].
    pub fn builder() -> GatewayBuilder {
        GatewayBuilder::new()
    }

    /// Wait for the next Revolt event.
    ///
    /// # Errors
    ///
    /// Returns the error type [`ReceiveErrorKind::Reconnect`] if it failed to
    /// connect to the API.
    ///
    /// Returns the error type [`ReceiveErrorKind::Io`] if an error occurred
    /// with the current Websocket connection.
    ///
    /// Returns the error type [`ReceiveErrorKind::Deserialize`] if the incoming
    /// event failed to deserialize.
    ///
    /// Returns an error type [`ReceiveErrorKind::Io`] if an outgoing event,
    /// such as authentication or heartbeat, could not be sent.
    ///
    /// [`ReceiveErrorKind`]: crate::error::ReceiveErrorKind
    #[instrument(skip(self))]
    pub async fn next_event(&mut self) -> Result<ServerEvent, ReceiveError> {
        /// The next action client should handle.
        #[derive(Debug)]
        enum Action {
            /// Establish a new connection.
            Connect,
            /// Send an authentication event.
            Authenticate,
            /// Send a heartbeat event.
            Heartbeat,
            /// Handle an incoming message from socket.
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
                    // we don't need to send close packet because
                    // tokio-websocket does it internally.
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

    /// Send a client event.
    ///
    /// # Errors
    ///
    /// Returns the error type [`SendErrorKind::Send`] if it failed to send a
    /// connection close  message, either if the connection is already closed or
    /// if sending a message over the socket failed.
    ///
    /// Returns a [`SendErrorKind::Serialize`] it if failed to serialize the
    /// event.
    ///
    /// [`SendErrorKind`]: crate::error::SendErrorKind
    pub async fn send(&mut self, event: &ClientEvent) -> Result<(), SendError> {
        self.socket
            .as_mut()
            .ok_or(SendError::new(SendErrorKind::Send, None))?
            .send(Self::encode_client_event(event)?)
            .await
            .map_err(|source| SendError::new(SendErrorKind::Send, Some(Box::new(source))))
    }

    /// Connect to the API.
    ///
    /// After connection sends an authentication event, if the token is
    /// provided.
    ///
    /// # Errors
    ///
    /// Returns a error type [`ReceiveErrorKind::Reconnect`] if it failed to
    /// connect to the API.
    ///
    /// [`ReceiveErrorKind`]: crate::error::ReceiveErrorKind
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

    /// Send a Websocket close mesaage.
    ///
    /// # Errors
    ///
    /// Returns the error type [`SendErrorKind::Send`] if it failed to send a
    /// connection close  message, either if the connection is already closed or
    /// if sending a message over the socket failed.
    ///
    /// [`SendErrorKind`]: crate::error::SendErrorKind
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

    /// Reset the connection state.
    fn disconnect(&mut self) {
        self.socket = None;
        self.heartbeat_interval = None;
    }

    /// Serialize the client event to outgoing Websocket message.
    ///
    /// # Errors
    ///
    /// Returns the error type [`SendErrorKind::Serialize`] if it failed to
    /// serialize the event.
    ///
    /// [`SendErrorKind`]: crate::error::SendErrorKind
    fn encode_client_event(event: &ClientEvent) -> Result<WsMessage, SendError> {
        serde_json::to_string(event)
            .map(WsMessage::text)
            .map_err(|source| SendError::new(SendErrorKind::Serialize, Some(Box::new(source))))
    }

    /// Deserialize an incoming message to server event.
    ///
    /// # Errors
    ///
    /// Returns the error type [`ReceiveErrorKind::Deserialize`] if it failed to
    /// deserialize the event.
    ///
    /// [`SendErrorKind`]: crate::error::SendErrorKind
    fn deserialize_server_event(value: &str) -> Result<ServerEvent, ReceiveError> {
        serde_json::from_str(value).map_err(|source| {
            ReceiveError::new(ReceiveErrorKind::Deserialize, Some(Box::new(source)))
        })
    }
}
