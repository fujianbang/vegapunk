use serde::Serialize;
use ssh2::{Channel, Session};
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;
use std::{collections::HashMap, io::Read};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{broadcast, Mutex};

use super::HostServerMessage;

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

    pub async fn close_connection(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let mut sessions = self.connection_sessions.lock().await;
        if let Some(connection) = sessions.get_mut(id) {
            connection.close().await?;
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

/// A SSH Connection
///
/// This struct is used to represent a SSH connection
pub struct Connection {
    // session to connect to ssh
    session: Session,
    // channel to send data to ssh
    channel: Arc<Mutex<Channel>>,
    // receiver to receive data from ssh
    tx: Sender<Vec<u8>>,
}

impl Connection {
    pub fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        session.userauth_password(username, password)?;

        let mut channel = session.channel_session()?;
        channel.request_pty("xterm", None, None)?;
        channel.shell()?;

        let channel = Arc::new(Mutex::new(channel));

        let (tx, mut rx) = broadcast::channel(100);

        let channel_clone = Arc::clone(&channel);

        let tx1 = tx.clone();

        tauri::async_runtime::spawn(async move {
            let mut buf = vec![0; 1024];
            let mut channel = channel_clone.lock().await;

            loop {
                match channel.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        if let Err(e) = tx1.send(buf[..n].to_vec()) {
                            eprintln!("Failed to send data: {}", e);
                        }
                    }
                    Ok(_) => break,
                    Err(e) => {
                        eprintln!("Error reading from channel: {}", e);
                        break;
                    }
                }
            }
        });

        tauri::async_runtime::spawn(async move {
            loop {
                let _ = rx.recv().await.unwrap();
            }
        });

        Ok(Self {
            session,
            channel,
            tx,
        })
    }

    /// Write data to the SSH channel
    async fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut channel = self.channel.lock().await;
        channel.write_all(data)?;
        channel.flush()?;
        Ok(())
    }

    pub fn subscribe(&self) -> Receiver<Vec<u8>> {
        self.tx.subscribe()
    }

    /// Close the SSH connection
    async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        self.channel.lock().await.close()?;
        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct TerminalOutput {
    data: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    fn get_test_credentials() -> (String, u16, String, String) {
        dotenv().ok();

        (
            env::var("TEST_SSH_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            env::var("TEST_SSH_PORT")
                .unwrap_or_else(|_| "22".to_string())
                .parse()
                .unwrap(),
            env::var("TEST_SSH_USER").unwrap_or_else(|_| "test".to_string()),
            env::var("TEST_SSH_PASSWORD").unwrap_or_else(|_| "password".to_string()),
        )
    }

    #[tokio::test]
    #[ignore]
    async fn test_successful_connection() {
        let (host, port, username, password) = get_test_credentials();
        println!("Credentials: {}:{} {}:***", host, port, username);
        let mut connection = Connection::new(&host, port, &username, &password)
            .expect("Failed to create connection");

        let mut rx = connection.subscribe();

        tauri::async_runtime::spawn(async move {
            loop {
                let data = rx.recv().await.unwrap();
                println!(
                    "got data from connection: {:?}",
                    String::from_utf8(data).unwrap()
                );
            }
        });

        connection.write("uname -a\n".as_bytes()).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
