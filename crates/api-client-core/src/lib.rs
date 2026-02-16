pub mod client;
pub mod error;
pub mod pagination;
pub mod websocket;

pub use client::{RestClient, RestClientBuilder};
pub use error::ApiClientError;
pub use pagination::paginate;
pub use websocket::WebSocketClient;
