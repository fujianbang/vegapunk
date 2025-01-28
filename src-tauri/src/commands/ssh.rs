use crate::ssh::{ConnectionManager, HostServerMessage};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
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
pub async fn listen_to_ssh(
    channel: Channel<HostServerMessage>,
    state: State<'_, Arc<ConnectionManager>>,
) -> Result<(), String> {
    // let manager = Arc::clone(&state);

    // let mut rx = manager.tx.subscribe();

    // tauri::async_runtime::spawn(async move {
    //     while let Ok(msg) = rx.recv().await {
    //         println!("ready to send: {:?}", msg);
    //         channel.send(msg).unwrap();
    //     }
    // });

    Ok(())
}

#[tauri::command]
pub async fn send_ssh_data(
    session_id: &str,
    data: String,
    state: State<'_, Arc<ConnectionManager>>,
) -> Result<(), String> {
    let manager = Arc::clone(&state);

    println!("got data[{}]: {:?}", session_id, data);

    manager
        .write_to_connection(session_id, data.as_bytes())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
