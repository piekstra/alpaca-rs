use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, warn};

use crate::error::ApiClientError;

/// Generic WebSocket client for streaming APIs.
///
/// Connects to a WebSocket endpoint, optionally sends an authentication message,
/// and provides a channel-based interface for receiving messages.
pub struct WebSocketClient {
    write: futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    receiver: mpsc::Receiver<Result<String, ApiClientError>>,
    _reader_handle: tokio::task::JoinHandle<()>,
}

impl WebSocketClient {
    /// Connect to a WebSocket endpoint.
    ///
    /// If `auth_message` is provided, it will be sent immediately after connection.
    pub async fn connect(
        url: &str,
        auth_message: Option<serde_json::Value>,
    ) -> Result<Self, ApiClientError> {
        debug!("WebSocket connecting to {url}");

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| ApiClientError::WebSocket(format!("Connection failed: {e}")))?;

        let (mut write, read) = ws_stream.split();

        if let Some(auth) = auth_message {
            let msg = serde_json::to_string(&auth)
                .map_err(|e| ApiClientError::WebSocket(format!("Auth serialization: {e}")))?;
            write
                .send(Message::Text(msg.into()))
                .await
                .map_err(|e| ApiClientError::WebSocket(format!("Auth send failed: {e}")))?;
            debug!("WebSocket auth message sent");
        }

        let (tx, rx) = mpsc::channel(256);

        let reader_handle = tokio::spawn(async move {
            let mut read = read;
            while let Some(msg_result) = read.next().await {
                match msg_result {
                    Ok(Message::Text(text)) => {
                        if tx.send(Ok(text.to_string())).await.is_err() {
                            break;
                        }
                    }
                    Ok(Message::Binary(data)) => match String::from_utf8(data.to_vec()) {
                        Ok(text) => {
                            if tx.send(Ok(text)).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            warn!("Non-UTF8 binary message: {e}");
                        }
                    },
                    Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => {}
                    Ok(Message::Close(_)) => {
                        debug!("WebSocket closed by server");
                        break;
                    }
                    Ok(Message::Frame(_)) => {}
                    Err(e) => {
                        error!("WebSocket read error: {e}");
                        let _ = tx
                            .send(Err(ApiClientError::WebSocket(format!("Read error: {e}"))))
                            .await;
                        break;
                    }
                }
            }
        });

        Ok(Self {
            write,
            receiver: rx,
            _reader_handle: reader_handle,
        })
    }

    /// Send a JSON message over the WebSocket.
    pub async fn send(&mut self, message: &serde_json::Value) -> Result<(), ApiClientError> {
        let text = serde_json::to_string(message)
            .map_err(|e| ApiClientError::WebSocket(format!("Serialization: {e}")))?;
        self.write
            .send(Message::Text(text.into()))
            .await
            .map_err(|e| ApiClientError::WebSocket(format!("Send failed: {e}")))
    }

    /// Receive the next message from the WebSocket.
    ///
    /// Returns `None` if the connection has been closed.
    pub async fn recv(&mut self) -> Option<Result<String, ApiClientError>> {
        self.receiver.recv().await
    }

    /// Receive and parse the next message as a typed JSON value.
    pub async fn recv_json<T: serde::de::DeserializeOwned>(
        &mut self,
    ) -> Option<Result<T, ApiClientError>> {
        match self.receiver.recv().await {
            Some(Ok(text)) => {
                Some(serde_json::from_str(&text).map_err(ApiClientError::Deserialize))
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Close the WebSocket connection.
    pub async fn close(mut self) -> Result<(), ApiClientError> {
        self.write
            .send(Message::Close(None))
            .await
            .map_err(|e| ApiClientError::WebSocket(format!("Close failed: {e}")))
    }
}
