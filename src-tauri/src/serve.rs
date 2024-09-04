use crate::{
    anime::{AnimeIndex, Anime},
    bangumi::{Bangumi},
    spider::Spider,
};

#[tauri::command()]
pub async fn get_list(key_word: String) -> Vec<AnimeIndex> {
    let anime_names = Bangumi::get_anime_index(&key_word).await.unwrap(); //TODO：错误处理！
    Bangumi::edit_distance_sort(anime_names, &key_word)
}

#[tauri::command()]
pub async fn get_info(anime_index: AnimeIndex) -> Anime {
    Bangumi::get_anime_info(anime_index).await.unwrap()
}