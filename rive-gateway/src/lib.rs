use async_channel::{self, Receiver, Sender};
use futures::{SinkExt, Stream, StreamExt};
use rive_models::event::{ClientToServerEvent, ServerToClientEvent};
use tokio::{net::TcpStream, select, spawn};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tungstenite error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Serde JSON deserialization/serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Client event sender error: {0}")]
    ClientSenderError(#[from] async_channel::SendError<ClientToServerEvent>),

    #[error("Server event sender error: {0}")]
    ServerSenderError(#[from] Box<async_channel::SendError<Result<ServerToClientEvent, Error>>>),
}

/// A wrapper for Revolt WebSocket API
#[derive(Debug, Clone)]
pub struct RevoltWs {
    client_sender: Sender<ClientToServerEvent>,
    server_receiver: Receiver<Result<ServerToClientEvent, Error>>,
}

impl RevoltWs {
    /// Connect to gateway with default Revolt WebSocket URL
    pub async fn connect() -> Result<Self, Error> {
        RevoltWs::connect_with_url("wss://ws.revolt.chat".to_string()).await
    }

    /// Connect to gateway with specified URL
    pub async fn connect_with_url(url: String) -> Result<Self, Error> {
        let (socket, _) = tokio_tungstenite::connect_async(url).await?;
        let (client_sender, client_receiver) = async_channel::unbounded();
        let (server_sender, server_receiver) = async_channel::unbounded();

        let revolt = RevoltWs {
            client_sender,
            server_receiver,
        };

        spawn(RevoltWs::handle(client_receiver, server_sender, socket));

        Ok(revolt)
    }

    /// Send an event to server
    pub async fn send(&self, event: ClientToServerEvent) -> Result<(), Error> {
        self.client_sender.send(event).await.map_err(Error::from)?;

        Ok(())
    }

    async fn handle(
        mut client_receiver: Receiver<ClientToServerEvent>,
        server_sender: Sender<Result<ServerToClientEvent, Error>>,
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

    fn encode_client_event(event: ClientToServerEvent) -> Result<Message, Error> {
        let json = serde_json::to_string(&event).map_err(Error::from)?;
        let msg = Message::Text(json);

        Ok(msg)
    }

    fn decode_server_event(msg: Message) -> Result<ServerToClientEvent, Error> {
        let text = msg.to_text().map_err(Error::from)?;
        let event = serde_json::from_str(text).map_err(Error::from)?;

        Ok(event)
    }
}

impl Stream for RevoltWs {
    type Item = Result<ServerToClientEvent, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.server_receiver.poll_next_unpin(cx)
    }
}