use serde::Deserialize;

use super::Person;

#[derive(Deserialize, Debug)]
pub struct Movie {
    pub title: String,
    pub id: usize,
    pub poster_path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Credits {
    pub id: usize,
    pub cast: Vec<Person>,
}
