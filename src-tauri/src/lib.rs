mod commands;
mod ssh;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::add_new_host,
            commands::get_hosts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
