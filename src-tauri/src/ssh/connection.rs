use serde::Serialize;
use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
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

        // open a interactive shell
        let mut channel = session.channel_session()?;
        channel.request_pty("xterm", None, None)?;
        channel.shell()?;

        Ok(Self {
            session: Arc::new(Mutex::new(session)),
            channel: Arc::new(Mutex::new(channel)),
        })
    }

    pub fn send_data(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        println!(
            "send data to ssh: \n-------------------------\n{:?}\n-------------------------\n",
            String::from_utf8_lossy(data)
        );
        let mut channel = self.channel.lock().unwrap();
        channel.write_all(data)?;
        channel.write_all(b"\n")?;
        channel.flush()?;
        Ok(())
    }

    pub fn start_read_loop(&self) {
        let channel = self.channel.clone();

        tauri::async_runtime::spawn(async move {
            let mut temp_buf = [0u8; 1024];

            loop {
                let mut channel = channel.lock().unwrap();
                println!("read data from ssh");
                match channel.read(&mut temp_buf) {
                    Ok(n) if n > 0 => {
                        let data = String::from_utf8_lossy(&temp_buf[..n]).to_string();

                        #[cfg(debug_assertions)]
                        println!(
                            "Read {} bytes\n-------------------------\n{}\n-------------------------\n",
                            n, data
                        );

                        // if let Err(e) = pty_channel.send(SshEvent::Data(data)) {
                        //     eprintln!("Failed to send data through channel: {}", e);
                        //     break;
                        // }
                    }
                    Ok(_) => {
                        println!("SSH channel closed");
                        break;
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::TimedOut {
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

        connection.send_data("ls -l".as_bytes()).unwrap();
        thread::sleep(Duration::from_secs(1));

        connection.send_data("uname -a".as_bytes()).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
