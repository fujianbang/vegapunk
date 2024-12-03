#[tauri::command]
pub fn add_new_host(address: String, port: u16, username: String, password: String) {
    println!("Adding new host: {}", address);
    println!("Port: {}", port);
    println!("Username: {}", username);
    println!("Password: {}", password);
}

struct Host {
    name: String,
    address: String,
    port: u16,
}
#[tauri::command]
pub fn get_hosts() -> Vec<Host> {
    vec![
        Host {
            name: "Host 1".to_string(),
            address: "192.168.1.1".to_string(),
            port: 22,
        },
        Host {
            name: "Host 2".to_string(),
            address: "192.168.1.2".to_string(),
            port: 22,
        },
    ]
}
