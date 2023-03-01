use reqwest::Result;
use serde::Deserialize;
use crate::Tmdb;

use super::{status::Status, Credits};

#[derive(Deserialize, Debug)]
pub struct Movie {
    pub title: String,
    pub id: usize,
    pub adult: bool,
    pub backdrop_path: Option<String>,
    // belongs_to_collection
    pub budget: usize,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub imdb_id: Option<String>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<ProductionCompanyResult>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: String,
    pub revenue: usize,
    pub runtime: Option<usize>,
    pub spoken_languages: Vec<Language>,
    pub status: Status,
    pub tagline: Option<String>,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: usize,
}

impl Movie {
    pub async fn get_credits(&self, tmdb: &Tmdb) -> Result<Credits> {
        tmdb.movies.get_credits(self.id).await
    } 
}

#[derive(Deserialize, Debug)]
pub struct ProductionCompanyResult {
    pub name: String,
    pub id: usize,
    pub logo_path: Option<String>,
    pub origin_country: String,
}

#[derive(Deserialize, Debug)]
pub struct Genre {
    pub name: String,
    pub id: usize,
}

#[derive(Deserialize, Debug)]
pub struct ProductionCountry {
    pub name: String,
    pub iso_3166_1: String,
}

#[derive(Deserialize, Debug)]
pub struct Language {
    pub name: String,
    pub iso_639_1: String,
}