use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum HostOS {
    Windows = 0,
    Apple = 1,
    Android = 2,
    Ubuntu = 3,
    Fedor = 4,
    OpenSUSE = 5,
    CentOS = 6,
    ArchLinux = 7,
    Debian = 8,
    RedHat = 9,
    Kali = 10,
    RockyLinux = 11,
    Unknown = 99,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    id: String,
    name: String,
    address: String,
    port: u16,
    auth_method: AuthMethod,
    os: HostOS,
    comment: String,
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
