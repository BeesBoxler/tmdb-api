use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ExternalIds {
    pub id: usize,
    pub imdb_id: Option<String>,
    pub facebook_id: Option<String>,
    pub instagram_id: Option<String>,
    pub twitter_id: Option<String>,
}