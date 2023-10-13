#![doc = include_str!("../README.md")]

pub mod error;

use error::{ReceiveError, ReceiveErrorKind, SendError, SendErrorKind};
use futures::{SinkExt, StreamExt};
use rive_models::{
    authentication::Authentication,
    event::{ClientEvent, ServerEvent},
};
use tokio::net::TcpStream;
use tokio_websockets::{MaybeTlsStream, Message as WsMessage, WebsocketStream};

type Socket = WebsocketStream<MaybeTlsStream<TcpStream>>;

/// Base WebSocket API URL of official Revolt instance
pub const BASE_URL: &str = "wss://ws.revolt.chat";

/// Gateway configuration
// TODO: rename to `Config`
#[derive(Debug, Clone)]
pub struct Config {
    /// Auth token. If it is not [`Authentication::None`] then the event will be sent automatically.
    pub auth: Authentication,
    /// WebSocket API base URL
    pub base_url: String,
    /// Whether auto heartbeat is enabled
    pub heartbeat: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auth: Authentication::None,
            base_url: BASE_URL.to_string(),
            heartbeat: true,
        }
    }
}

impl Config {
    /// Creates a new [`GatewayConfig`].
    pub fn new(auth: Authentication, base_url: String, heartbeat: bool) -> Self {
        Self {
            auth,
            base_url,
            heartbeat,
        }
    }
}

/// A wrapper for Revolt WebSocket API
// TODO: config builder
#[derive(Debug)]
pub struct Gateway {
    socket: Option<Socket>,
    config: Config,
}

impl Gateway {
    pub fn new(auth: Authentication) -> Self {
        Self::with_url(BASE_URL, auth)
    }

    pub fn with_url(url: impl Into<String>, auth: Authentication) -> Self {
        Self::with_config(Config::new(auth, url.into(), true))
    }

    pub fn with_config(config: Config) -> Self {
        Self {
            socket: None,
            config,
        }
    }

    pub async fn next_event(&mut self) -> Result<ServerEvent, ReceiveError> {
        match self.socket {
            Some(_) => {}
            None => {
                println!("connection is none, connecting...");
                self.connect().await?;
            }
        };

        let socket = self
            .socket
            .as_mut()
            .ok_or(ReceiveError::new(ReceiveErrorKind::Reconnect, None))?;

        match socket.next().await {
            Some(res) => res
                .map(|msg| {
                    println!("got message");
                    Self::decode_server_event(msg)
                })
                .map_err(|err| ReceiveError::new(ReceiveErrorKind::Io, Some(Box::new(err))))?,
            None => {
                println!("received none, disconnecting");
                self.reset();
                Err(ReceiveError::new(ReceiveErrorKind::Io, None))
            }
        }
    }

    pub async fn send(&mut self, event: &ClientEvent) -> Result<(), SendError> {
        self.socket
            .as_mut()
            .ok_or(SendError::new(SendErrorKind::Send, None))?
            .send(Self::encode_client_event(event)?)
            .await
            .map_err(|source| SendError::new(SendErrorKind::Send, Some(Box::new(source))))?;

        Ok(())
    }

    async fn connect(&mut self) -> Result<(), ReceiveError> {
        let (socket, _) = tokio_websockets::ClientBuilder::from_uri(
            self.config.base_url.clone().try_into().expect("valid url"),
        )
        .connect()
        .await
        .map_err(|source| ReceiveError::new(ReceiveErrorKind::Reconnect, Some(Box::new(source))))?;
        println!("connected");

        self.socket = Some(socket);

        self.send(&ClientEvent::Authenticate {
            token: self.config.auth.value(),
        })
        .await
        .map_err(|source| ReceiveError::from_send(source))?;
        println!("sent auth");

        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), SendError> {
        self.socket
            .as_mut()
            .ok_or(SendError::new(SendErrorKind::Send, None))?
            .close()
            .await
            .map_err(|source| SendError::new(SendErrorKind::Send, Some(Box::new(source))))?;
        self.reset();
        Ok(())
    }

    // async fn reconnect(&mut self) -> Result<(), ReceiveError> {
    //     self.disconnect().await.map_err(|source| {
    //         ReceiveError::new(ReceiveErrorKind::SendMessage, Some(Box::new(source)))
    //     })?;
    //     self.connect().await?;
    //     Ok(())
    // }

    fn reset(&mut self) {
        self.socket = None;
    }

    fn encode_client_event(event: &ClientEvent) -> Result<WsMessage, SendError> {
        serde_json::to_string(event)
            .map(WsMessage::text)
            .map_err(|source| SendError::new(SendErrorKind::Serialize, Some(Box::new(source))))
    }

    fn decode_server_event(msg: WsMessage) -> Result<ServerEvent, ReceiveError> {
        serde_json::from_str(msg.as_text().expect("message is text")).map_err(|source| {
            ReceiveError::new(ReceiveErrorKind::Deserialize, Some(Box::new(source)))
        })
    }
}
