mod anime;
mod bangumi;
mod serve;
mod spider;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            serve::net_is_ok,
            serve::get_id,
            serve::get_anime_index,
            serve::get_anime_info,
            serve::show_main_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
