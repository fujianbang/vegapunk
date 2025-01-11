mod commands;
mod ssh;

use ssh::create_connection_manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let connection_manager = create_connection_manager();

    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_shell::init())
        .manage(connection_manager)
        .invoke_handler(tauri::generate_handler![
            commands::add_new_host,
            commands::get_hosts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
