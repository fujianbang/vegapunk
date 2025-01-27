use serde::Serialize;
use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{channel as mpsc_channel, Sender as MpscSender};
use std::sync::{Arc, Mutex};
use tauri::ipc::Channel;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SshEvent {
    Data(String),
}

/// SSH Connection
pub struct Connection {
    session: Arc<Mutex<Session>>,
    channel: Arc<Mutex<ssh2::Channel>>,
    write_tx: MpscSender<Vec<u8>>,
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

        let (write_tx, write_rx) = mpsc_channel();
        let channel_clone = Arc::new(Mutex::new(channel));
        let write_channel = channel_clone.clone();

        tauri::async_runtime::spawn(async move {
            while let Ok(data) = write_rx.recv() {
                let mut channel = write_channel.lock().unwrap();
                if let Err(e) = channel.write_all(&data) {
                    eprintln!("Failed to write to SSH channel: {}", e);
                    break;
                }
                if let Err(e) = channel.flush() {
                    eprintln!("Failed to flush SSH channel: {}", e);
                    break;
                }
            }
        });

        Ok(Self {
            session: Arc::new(Mutex::new(session)),
            channel: channel_clone,
            write_tx,
        })
    }

    pub fn send_data(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        println!(
            "send data to ssh: \n-------------------------\n{:?}\n-------------------------\n",
            String::from_utf8_lossy(data)
        );
        self.write_tx.send(data.to_vec())?;
        Ok(())
    }

    pub fn start_read_loop(&self) {
        let channel = self.channel.clone();

        tauri::async_runtime::spawn(async move {
            let mut temp_buf = [0u8; 1024];

            loop {
                let mut channel = channel.lock().unwrap();
                match channel.read(&mut temp_buf) {
                    Ok(n) if n > 0 => {
                        let data = String::from_utf8_lossy(&temp_buf[..n]).to_string();
                        #[cfg(debug_assertions)]
                        println!(
                            "Read {} bytes\n-------------------------\n{}\n-------------------------\n",
                            n, data
                        );
                    }
                    Ok(0) => {
                        println!("SSH channel closed");
                        break;
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::TimedOut {
                            drop(channel);
                            std::thread::sleep(std::time::Duration::from_millis(10));
                            continue;
                        }
                        eprintln!("Error reading SSH channel: {}", e);
                        break;
                    }
                }
            }
        });
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
    use std::thread;
    use std::time::Duration;

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

    #[test]
    #[ignore]
    fn test_successful_connection() {
        let (host, port, username, password) = get_test_credentials();
        println!("Credentials: {}:{} {}:***", host, port, username);
        let result = Connection::new(&host, port, &username, &password);
        assert!(result.is_ok());

        let connection = result.unwrap();
        // read data from ssh
        connection.start_read_loop();

        connection.send_data("ls -l\n".as_bytes()).unwrap();
        thread::sleep(Duration::from_secs(1));

        connection.send_data("uname -a\n".as_bytes()).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
