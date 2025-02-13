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
