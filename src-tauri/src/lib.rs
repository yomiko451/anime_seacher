mod anime;
mod bangumi;
mod serve;
mod spider;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            serve::get_list,
            serve::get_info
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
