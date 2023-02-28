use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AlternativeTitles {
    pub id: usize,
    pub titles: Vec<Title>,
}

#[derive(Deserialize, Debug)]
pub struct Title {
    pub iso_3166_1: String,
    pub title: String,
    #[serde(rename(deserialize = "type"))]
    pub title_type: String,
}