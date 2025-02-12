mod connection;
mod error;

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::ssh::connection::Connection;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Server2ClientMsg {
    Connected { id: String },
    Disconnected { id: String },
    Data(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Client2ServerMsg {
    Connect,
    Disconnect { id: String },
    SendData { id: String, data: String },
}


pub struct ConnectionManager {
    // connection id -> connection
    connection_sessions: Arc<Mutex<HashMap<String, Connection>>>,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new() -> Self {
        Self {
            connection_sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new connection
    pub async fn create_connection(
        &self,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        let connection = Connection::new(host, port, username, password)?;

        let id = uuid::Uuid::new_v4().to_string();

        // add connection to manager
        self.connection_sessions
            .lock()
            .await
            .insert(id.clone(), connection);

        Ok(())
    }

    pub async fn write_to_connection(&self, id: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut sessions = self.connection_sessions.lock().await;

        if let Some(connection) = sessions.get_mut(id) {
            connection.write(data).await?;
        }
        Ok(())
    }

    // pub async fn get_connection_rx(&self, id: &str) -> Result<Receiver<Vec<u8>>, Box<dyn Error>> {
    //     let mut sessions = self.connection_sessions.lock().await;
    //     if let Some(connection) = sessions.get(id) {
    //         Ok(connection.rx.subscribe())
    //     } else {
    //         Err(format!("Connection {} not found", id).into())
    //     }
    // }
}