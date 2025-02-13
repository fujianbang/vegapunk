use std::sync::Arc;

use ssh::ConnectionManager;
use tauri::Manager;

mod commands;
mod ssh;
mod encrypt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Arc::new(ConnectionManager::new()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_new_host,
            commands::get_hosts,
            commands::ssh::communicate,
            commands::ssh::listen_ssh_data,
            commands::ssh::create_ssh_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
