mod connection;
mod manager;

pub use connection::{SSHConnection, SSHError};
pub use manager::{ConnectionManager, SharedConnectionManager, create_connection_manager}; 

#[cfg(test)]
pub mod test_utils {
    use dotenv::dotenv;
    
    pub fn setup() {
        dotenv().ok();
    }
} 