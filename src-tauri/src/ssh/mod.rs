mod connection;
mod error;

pub use connection::{Connection, SshEvent};

#[cfg(test)]
pub mod test_utils {
    use dotenv::dotenv;

    pub fn setup() {
        dotenv().ok();
    }
}
