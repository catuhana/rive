#![doc = include_str!("../README.md")]

use std::time::Duration;

use async_channel::{self, Receiver, Sender};
use futures::{SinkExt, Stream, StreamExt};
use rive_models::{
    authentication::Authentication,
    event::{ClientEvent, ServerEvent},
};
use tokio::{net::TcpStream, select, spawn, time::sleep};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

/// Base WebSocket API URL of official Revolt instance
pub const BASE_URL: &str = "wss://ws.revolt.chat";

/// Gateway client error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// WebSocket error
    #[error("Tungstenite error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),

    /// Data serialization/deserialization error
    #[error("Serde JSON deserialization/serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Internal client event channel sender error
    #[error("Client event sender error: {0}")]
    ClientSenderError(#[from] async_channel::SendError<ClientEvent>),

    /// Internal server event channel sender error
    #[error("Server event sender error: {0}")]
    ServerSenderError(#[from] Box<async_channel::SendError<Result<ServerEvent, Error>>>),
}

/// Gateway configuration
// TODO: rename to `Config`
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    /// Auth token. If it is not [`Authentication::None`] then the event will be sent automatically.
    pub auth: Authentication,
    /// WebSocket API base URL
    pub base_url: String,
    /// Whether auto heartbeat is enabled
    pub heartbeat: bool,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            auth: Authentication::None,
            base_url: BASE_URL.to_string(),
            heartbeat: true,
        }
    }
}

impl GatewayConfig {
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
#[derive(Debug, Clone)]
pub struct Gateway {
    client_sender: Sender<ClientEvent>,
    server_receiver: Receiver<Result<ServerEvent, Error>>,
}

impl Gateway {
    /// Connect to gateway with default Revolt WebSocket URL ([`BASE_URL`])
    pub async fn connect(auth: Authentication) -> Result<Self, Error> {
        Gateway::connect_with_url(BASE_URL, auth).await
    }

    /// Connect to gateway with specified URL
    pub async fn connect_with_url(
        url: impl Into<String>,
        auth: Authentication,
    ) -> Result<Self, Error> {
        Self::connect_with_config(GatewayConfig::new(auth, url.into(), true)).await
    }

    pub async fn connect_with_config(config: GatewayConfig) -> Result<Self, Error> {
        let (socket, _) = tokio_tungstenite::connect_async(&config.base_url).await?;
        let (client_sender, client_receiver) = async_channel::unbounded();
        let (server_sender, server_receiver) = async_channel::unbounded();

        let revolt = Gateway {
            client_sender: client_sender.clone(),
            server_receiver,
        };

        spawn(Gateway::handle(client_receiver, server_sender, socket));

        if config.heartbeat {
            spawn(Self::heartbeat(client_sender));
        }

        if !matches!(config.auth, Authentication::None) {
            let event = ClientEvent::Authenticate {
                token: config.auth.value(),
            };
            revolt.send(event).await?;
        }

        Ok(revolt)
    }

    /// Send an event to server
    pub async fn send(&self, event: ClientEvent) -> Result<(), Error> {
        self.client_sender.send(event).await.map_err(Error::from)?;

        Ok(())
    }

    async fn heartbeat(client_sender: Sender<ClientEvent>) -> Result<(), Error> {
        loop {
            // TODO: an ability to send custom value somehow
            // it can be useful for ping measure for example
            client_sender.send(ClientEvent::Ping { data: 0 }).await?;
            sleep(Duration::from_secs(15)).await;
        }
    }

    async fn handle(
        mut client_receiver: Receiver<ClientEvent>,
        server_sender: Sender<Result<ServerEvent, Error>>,
        mut socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> Result<(), Error> {
        loop {
            select! {
                Some(event) = client_receiver.next() => {
                    let msg = Self::encode_client_event(event)?;
                    socket.send(msg).await?;
                },
                Some(msg) = socket.next() => {
                    let msg = msg.map_err(Error::from)?;
                    let event = Self::decode_server_event(msg);
                    server_sender.send(event).await.map_err(|err| Error::from(Box::new(err)))?;
                },
                else => break,
            };
        }

        Ok(())
    }

    fn encode_client_event(event: ClientEvent) -> Result<Message, Error> {
        let json = serde_json::to_string(&event).map_err(Error::from)?;
        let msg = Message::Text(json);

        Ok(msg)
    }

    fn decode_server_event(msg: Message) -> Result<ServerEvent, Error> {
        let text = msg.to_text().map_err(Error::from)?;
        let event = serde_json::from_str(text).map_err(Error::from)?;

        Ok(event)
    }
}

impl Stream for Gateway {
    type Item = Result<ServerEvent, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.server_receiver.poll_next_unpin(cx)
    }
}
