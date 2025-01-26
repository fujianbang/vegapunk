use std::sync::Mutex;

use tauri::Manager;

mod commands;
mod ssh;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(Mutex::new(commands::SSHState { connection: None }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_new_host,
            commands::get_hosts,
            commands::ssh::create_ssh_connection,
            commands::ssh::listen_ssh_data,
            commands::ssh::send_ssh_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
