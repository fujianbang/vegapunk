use serde::Serialize;
use ssh2::{Channel, Session};
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;
use std::{collections::HashMap, io::Read};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{broadcast, Mutex};

use super::Server2ClientMsg;

/// SSH Connection
///
/// This struct is used to represent an SSH connection
pub struct Connection {
    // session to connect to ssh
    session: Session,
    // channel to send data to ssh
    channel: Arc<Mutex<Channel>>,
}

impl Connection {
    pub(crate) fn new(
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

        assert!(session.authenticated());

        let mut channel = session.channel_session()?;
        channel.request_pty("xterm", None, None)?;
        channel.shell()?;

        Ok(Self {
            session,
            channel: Arc::new(Mutex::new(channel)),
        })
    }

    /// Write data to the SSH channel
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut channel = self.channel.lock().await;
        channel.write_all(data)?;
        channel.flush()?;
        Ok(())
    }

    /// open a channel
    pub(crate) async fn channel(&self) {
        let channel_clone = self.channel.clone();

        let mut channel = channel_clone.lock().await;

        let mut buf = vec![0; 1024];

        loop {
            match channel.read(&mut buf) {
                Ok(n) if n > 0 => {
                    // if let Err(e) = tx.send(buf[..n].to_vec()) {
                    //     eprintln!("Failed to send data: {}", e);
                    // }
                    print!("{}", String::from_utf8_lossy(&buf[..n]));
                }
                Ok(_) => break,
                Err(e) => {
                    eprintln!("Error reading from channel: {}", e);
                    break;
                }
            }
        }
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
