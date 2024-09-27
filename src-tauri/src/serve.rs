use tauri::Window;


use crate::{
    anime::{Anime, AnimeIndex},
    bangumi::Bangumi,
    spider::Spider,
};

type Source = Bangumi;

#[tauri::command()]
pub fn net_is_ok() -> bool {
    Source::net_is_ok()
}

#[tauri::command()]
pub fn get_id() -> String {
    Source::get_id()
}

#[tauri::command()]
pub async fn get_anime_index(key_word: String) -> Result<Vec<AnimeIndex>, String> {
    Bangumi::get_anime_index(&key_word).await.map_err(|e|e.to_string())
}

#[tauri::command()]
pub async fn get_anime_info(anime_index: AnimeIndex) -> Result<Anime, String> {
    Source::get_anime_info(anime_index).await.map_err(|e|e.to_string())
}

#[tauri::command()]
pub fn show_main_window(window: Window) {
    window.show().unwrap();
}