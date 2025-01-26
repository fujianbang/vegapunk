use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

/// SSH Connection
pub struct Connection {
    session: Session,
    channel: ssh2::Channel,
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

        Ok(Self { session, channel })
    }

    pub fn send_data(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.channel.write_all(data)?;
        self.channel.write_all(b"\n")?;
        self.channel.flush()?;
        Ok(())
    }

    pub fn read_output(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut temp_buf = [0u8; 1024];
        let mut buffer = Vec::new();

        // set read timeout, avoid infinite wait
        self.session.set_timeout(1000);

        loop {
            match self.channel.read(&mut temp_buf) {
                Ok(n @ 1..) => {
                    // append read data to buffer
                    buffer.extend_from_slice(&temp_buf[..n]);

                    #[cfg(debug_assertions)]
                    {
                        println!("Read {} bytes", n);
                        println!("{}", String::from_utf8_lossy(&temp_buf[..n]));
                    }
                }
                Ok(0) => break,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        break;
                    }
                    return Err(Box::new(e));
                }
            }
        }

        Ok(buffer)
    }
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
    }

    #[test]
    #[ignore]
    fn test_command_execution() {
        let (host, port, username, password) = get_test_credentials();
        println!("Credentials: {}:{} {}:***", host, port, username);
        let mut connection = Connection::new(&host, port, &username, &password).unwrap();

        let _ = connection.send_data(b"uname -a");

        thread::sleep(Duration::from_millis(500));

        match connection.read_output() {
            Ok(output) => println!("Output: {}", String::from_utf8_lossy(&output)),
            Err(e) => println!("Error reading output: {}", e),
        }
    }
}
