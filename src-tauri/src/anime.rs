use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Anime {
    pub title: String,
    pub description: String,
    pub staff: Vec<(String, String)>,
    pub cast: Vec<(String, String)>,
    pub cover: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnimeIndex {
    pub name: String,
    pub id: String,
    pub edit_distance: usize,
}
