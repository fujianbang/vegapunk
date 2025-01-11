use ssh2::Session;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct SSHConnection {
    session: Session,
    channel: ssh2::Channel,
}

#[derive(Debug)]
pub enum SSHError {
    IoError(io::Error),
    SSHError(ssh2::Error),
    ConnectionError(String),
}

impl From<io::Error> for SSHError {
    fn from(error: io::Error) -> Self {
        SSHError::IoError(error)
    }
}

impl From<ssh2::Error> for SSHError {
    fn from(error: ssh2::Error) -> Self {
        SSHError::SSHError(error)
    }
}

impl SSHConnection {
    pub fn new(host: &str, port: u16, username: &str, password: &str) -> Result<Self, SSHError> {
        let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
        let mut session = Session::new().map_err(SSHError::SSHError)?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        session.userauth_password(username, password)?;

        let channel = session.channel_session()?;

        Ok(Self { session, channel })
    }

    pub fn execute_command(&mut self, command: &str) -> Result<String, SSHError> {
        self.channel.exec(command)?;
        let mut output = String::new();
        self.channel.read_to_string(&mut output)?;
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn get_test_credentials() -> (String, u16, String, String) {
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
    fn test_connection_error_handling() {
        let result = SSHConnection::new("invalid-host", 22, "invalid-user", "invalid-password");
        assert!(result.is_err());

        if let Err(err) = result {
            match err {
                SSHError::IoError(_) => (), // 期望的错误类型
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }

    #[test]
    #[ignore] // 需要有效的SSH服务器才能运行
    fn test_successful_connection() {
        let (host, port, username, password) = get_test_credentials();
        let result = SSHConnection::new(&host, port, &username, &password);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore] // 需要有效的SSH服务器才能运行
    fn test_command_execution() {
        let (host, port, username, password) = get_test_credentials();
        let mut connection = SSHConnection::new(&host, port, &username, &password).unwrap();

        let result = connection.execute_command("echo 'test'");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "test");
    }
}
