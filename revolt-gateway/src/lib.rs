use futures::{SinkExt, Stream, StreamExt};
use revolt_models::event::{ClientToServerEvent, ServerToClientEvent};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tungstenite error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Serde JSON deserialization/serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// A wrapper for Revolt WebSocket API
#[derive(Debug)]
pub struct RevoltWs {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl RevoltWs {
    /// Connect to gateway with default Revolt WebSocket URL
    pub async fn connect() -> Result<Self, Error> {
        RevoltWs::connect_with_url("wss://ws.revolt.chat".to_string()).await
    }

    /// Connect to gateway with specified URL
    pub async fn connect_with_url(url: String) -> Result<Self, Error> {
        let (socket, _) = tokio_tungstenite::connect_async(url).await?;

        Ok(RevoltWs { socket })
    }

    /// Send an event to server
    pub async fn send(&mut self, event: ClientToServerEvent) -> Result<(), Error> {
        let msg = Self::encode_client_event(event)?;
        self.socket.send(msg).await.map_err(Error::from)?;

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
        self.socket
            .poll_next_unpin(cx)
            .map_ok(Self::decode_server_event)
            .map_err(Error::from)?
    }
}
