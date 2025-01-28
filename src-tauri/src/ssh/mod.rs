mod connection;
mod error;

pub use connection::ConnectionManager;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum HostServerMessage {
    Connected { id: String },
    Disconnected { id: String },
    DataReceived(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum HostClientMessage {
    Connect,
    Disconnect { id: String },
    SendData { id: String, data: String },
}
