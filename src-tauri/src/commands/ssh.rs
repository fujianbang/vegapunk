use crate::ssh::{ConnectionManager, Server2ClientMsg};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::ipc::Channel;
use tauri::State;
use tokio::time::{sleep, Duration};

fn get_test_credentials() -> (String, u16, String, String) {
    (
        "127.0.0.1".to_string(),
        22,
        "root".to_string(),
        "123456".to_string(),
    )
}

#[tauri::command]
pub async fn create_ssh_connection(state: State<'_, Arc<ConnectionManager>>) -> Result<(), String> {
    let (host, port, username, password) = get_test_credentials();
    println!("Credentials: {}:{} {}:***", host, port, username);

    let manager = Arc::clone(&state);
    manager
        .create_connection(&host, port, &username, &password)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn listen_ssh_data(session_id: String, channel: Channel<Server2ClientMsg>) {
    println!("Got listener: {}", session_id);

    loop {
        let msg = format!("{}\r\n", chrono::Local::now());
        channel.send(Server2ClientMsg::Data(msg)).unwrap();
        sleep(Duration::from_secs(2)).await;
    }
}

#[tauri::command]
pub async fn communicate(
    session_id: String,
    data: String,
    state: State<'_, Arc<ConnectionManager>>,
) -> Result<(), String> {
    // let manager = Arc::clone(&state);

    println!("got data[{}]: {:?}", session_id, data);

    Ok(())
}
