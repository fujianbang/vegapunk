mod message;

use crate::commands::message::{AuthMethod, Host, HostOS};
use serde::{Deserialize, Serialize};

/**
export interface AddNewHostRequest {
    name: string;
    address: string;
    port: number;
    auth_method: HostAuthMethod;
    comment: string;
}
**/

pub enum SSHAuthorizeMethod {
    Password(String),
    PublicKey(String),
}
pub struct AddNewHostRequest {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub auth_method: SSHAuthorizeMethod,
}

#[tauri::command]
pub fn add_new_host(name: String, address: String, port: u16, username: String, password: String) {
    println!("Adding new host({}): {}", name, address);
    println!("Port: {}", port);
    println!("Username: {}", username);
    println!("Password: {}", password);
}

#[tauri::command]
pub fn get_hosts() -> Vec<Host> {
    vec![
        Host {
            id: "1".to_string(),
            name: "Host 1".to_string(),
            address: "192.168.1.1".to_string(),
            port: 22,
            auth_method: AuthMethod::Password,
            os: HostOS::Ubuntu,
            comment: "This is a comment".to_string(),
        },
        Host {
            id: "2".to_string(),
            name: "Host 2".to_string(),
            address: "192.168.1.2".to_string(),
            port: 22,
            auth_method: AuthMethod::Password,
            os: HostOS::Windows,
            comment: "This is another comment".to_string(),
        },
        Host {
            id: "3".to_string(),
            name: "Host 3".to_string(),
            address: "192.168.1.3".to_string(),
            port: 60022,
            auth_method: AuthMethod::Password,
            os: HostOS::Apple,
            comment: "This is yet another comment".to_string(),
        },
    ]
}
