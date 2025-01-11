mod connection;
mod error;

pub use error::SSHError;

#[cfg(test)]
pub mod test_utils {
    use dotenv::dotenv;

    pub fn setup() {
        dotenv().ok();
    }
}
