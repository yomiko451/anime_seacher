use serde::Serialize;

use crate::anime::{Anime, AnimeIndex};

pub trait Spider {
    fn get_id() -> String;

    fn net_check() -> anyhow::Result<()>;

    async fn get_anime_index(key_words: &str) -> anyhow::Result<Vec<AnimeIndex>>;

    async fn get_anime_info(anime_index: AnimeIndex) -> anyhow::Result<Anime>;

    fn edit_distance_sort(mut anime_names: Vec<AnimeIndex>, key_word: &str) -> Vec<AnimeIndex> {
        for item in &mut anime_names {
            item.edit_distance = edit_distance::edit_distance(key_word, &item.name);
        }
        anime_names.sort_by(|a, b| a.edit_distance.partial_cmp(&b.edit_distance).unwrap());

        anime_names
    }
}
