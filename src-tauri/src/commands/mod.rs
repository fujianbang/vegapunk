use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[tauri::command]
pub fn add_new_host(address: String, port: u16, username: String, password: String) {
    println!("Adding new host: {}", address);
    println!("Port: {}", port);
    println!("Username: {}", username);
    println!("Password: {}", password);
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum AuthMethod {
    Password = 0,
    PublicKey = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    name: String,
    address: String,
    port: u16,
    auth_method: AuthMethod,
    comment: String,
}
#[tauri::command]
pub fn get_hosts() -> Vec<Host> {
    vec![
        Host {
            name: "Host 1".to_string(),
            address: "192.168.1.1".to_string(),
            port: 22,
            auth_method: AuthMethod::Password,
            comment: "This is a comment".to_string(),
        },
        Host {
            name: "Host 2".to_string(),
            address: "192.168.1.2".to_string(),
            port: 22,
            auth_method: AuthMethod::Password,
            comment: "This is another comment".to_string(),
        },
    ]
}
