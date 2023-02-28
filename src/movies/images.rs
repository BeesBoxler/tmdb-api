use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Images {
    pub id: usize,
    pub backdrops: Vec<Image>,
    pub posters: Vec<Image>,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub aspect_ratio: f64,
    pub file_path: String,
    pub height: usize,
    pub iso_639_1: Option<String>,
    pub vote_average: f64,
    pub vote_count: usize,
    pub width: usize,
}