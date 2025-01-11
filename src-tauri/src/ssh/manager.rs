use super::connection::{SSHConnection, SSHError};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::async_runtime::Mutex;
use uuid::Uuid;

pub struct ConnectionManager {
    connections: HashMap<String, SSHConnection>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub fn create_connection(
        &mut self,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<String, SSHError> {
        let connection = SSHConnection::new(host, port, username, password)?;
        let id = Uuid::new_v4().to_string();
        self.connections.insert(id.clone(), connection);
        Ok(id)
    }

    pub fn execute_command(
        &mut self,
        connection_id: &str,
        command: &str,
    ) -> Result<String, SSHError> {
        self.connections
            .get_mut(connection_id)
            .ok_or(SSHError::ConnectionError("Connection not found".into()))?
            .execute_command(command)
    }

    pub fn remove_connection(&mut self, connection_id: &str) {
        self.connections.remove(connection_id);
    }
}

pub type SharedConnectionManager = Arc<Mutex<ConnectionManager>>;

pub fn create_connection_manager() -> SharedConnectionManager {
    Arc::new(Mutex::new(ConnectionManager::new()))
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
    fn test_manager_creation() {
        let manager = ConnectionManager::new();
        assert!(manager.connections.is_empty());
    }

    #[test]
    #[ignore] // 需要有效的SSH服务器才能运行
    fn test_connection_lifecycle() {
        let mut manager = ConnectionManager::new();
        let (host, port, username, password) = get_test_credentials();

        // 测试创建连接
        let connection_id = manager
            .create_connection(&host, port, &username, &password)
            .expect("Failed to create connection");
        assert_eq!(manager.connections.len(), 1);

        // 测试执行命令
        let result = manager.execute_command(&connection_id, "echo 'test'");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "test");

        // 测试移除连接
        manager.remove_connection(&connection_id);
        assert!(manager.connections.is_empty());
    }

    #[test]
    fn test_invalid_connection_id() {
        let mut manager = ConnectionManager::new();
        let result = manager.execute_command("invalid-id", "echo 'test'");
        assert!(result.is_err());

        if let Err(err) = result {
            match err {
                SSHError::ConnectionError(_) => (), // 期望的错误类型
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }

    #[test]
    fn test_shared_manager() {
        let shared_manager = create_connection_manager();
        assert!(shared_manager.try_lock().is_ok());
    }
}
