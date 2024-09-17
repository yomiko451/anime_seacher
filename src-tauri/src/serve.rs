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
pub async fn get_anime_index(key_word: String) -> Option<Vec<AnimeIndex>> {
    let anime_names = Bangumi::get_anime_index(&key_word).await;
    match anime_names {
        Ok(result) => Some(Source::edit_distance_sort(result, &key_word)),
        Err(_) => None
    }
}

#[tauri::command()]
pub async fn get_anime_info(anime_index: AnimeIndex) -> Option<Anime> {
    Source::get_anime_info(anime_index).await.ok()
}

#[tauri::command()]
pub fn show_main_window(window: Window) {
    window.show().unwrap();
}