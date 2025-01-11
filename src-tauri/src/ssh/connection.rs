use ssh2::Session;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

use super::SSHError;

/// SSH Connection
pub struct Connection {
    session: Session,
    channel: ssh2::Channel,
}

impl Connection {
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
        let mut connection = Connection::new(&host, port, &username, &password).unwrap();

        // let result = connection.execute_command("echo 'test'");
        // assert!(result.is_ok());
        // assert_eq!(result.unwrap().trim(), "test");

        // execute uname -a command
        let result = connection.execute_command("uname -a");
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
