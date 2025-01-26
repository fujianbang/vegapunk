use super::SSHState;
use crate::ssh::{Connection, SshEvent};
use std::sync::Mutex;
use tauri::ipc::Channel;
use tauri::State;

fn get_test_credentials() -> (String, u16, String, String) {
    (
        "127.0.0.1".to_string(),
        22,
        "root".to_string(),
        "123456".to_string(),
    )
}

#[tauri::command]
pub async fn create_ssh_connection(state: State<'_, Mutex<SSHState>>) -> Result<(), String> {
    let (host, port, username, password) = get_test_credentials();
    println!("Credentials: {}:{} {}:***", host, port, username);

    let connection =
        Connection::new(&host, port, &username, &password).map_err(|e| e.to_string())?;

    let mut state = state.lock().unwrap();

    state.connection = Some(connection);

    Ok(())
}

#[tauri::command]
pub async fn listen_ssh_data(
    state: State<'_, Mutex<SSHState>>,
    pty_channel: Channel<SshEvent>,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();

    println!("Connection listening started");
    if let Some(conn) = state.connection.take() {
        // 启动读取循环
        conn.start_read_loop();
    }
    println!("Connection listening closed");
    Ok(())
}

#[tauri::command]
pub async fn send_ssh_data(state: State<'_, Mutex<SSHState>>, data: String) -> Result<(), String> {
    let state = state.lock().unwrap();

    println!("send data: {}", data);

    if let Some(conn) = state.connection.as_ref() {
        conn.send_data(data.as_bytes()).map_err(|e| e.to_string())?;
    }

    Ok(())
}
