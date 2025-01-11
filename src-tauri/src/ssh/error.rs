use std::io;

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