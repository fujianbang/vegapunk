#[tauri::command]
pub fn add_new_host(address: String, port: u16, username: String, password: String) {
    println!("Adding new host: {}", address);
    println!("Port: {}", port);
    println!("Username: {}", username);
    println!("Password: {}", password);
}
