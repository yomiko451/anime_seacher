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
pub async fn get_anime_index(key_word: String) -> Vec<AnimeIndex> {
    let anime_names = Bangumi::get_anime_index(&key_word).await.unwrap(); //TODO：错误处理！
    Source::edit_distance_sort(anime_names, &key_word)
}

#[tauri::command()]
pub async fn get_anime_info(anime_index: AnimeIndex) -> Anime {
    Source::get_anime_info(anime_index).await.unwrap()
}
