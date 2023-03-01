use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MovieResults {
    pub page:usize,
    pub results: Vec<MovieResult>,
    pub total_pages: usize,
    pub total_results: usize,
}

#[derive(Deserialize, Debug)]
pub struct DatedMovieResults {
    pub page:usize,
    pub results: Vec<MovieResult>,
    pub total_pages: usize,
    pub total_results: usize,
    pub dates: MinMaxDates,
}

#[derive(Deserialize, Debug)]
pub struct MinMaxDates {
    pub maximum: String,
    pub minimum: String,
}

#[derive(Deserialize, Debug)]
pub struct MovieResult {
    pub poster_path: Option<String>,
    pub adult: bool,
    pub overview: String,
    pub release_date: String,
    pub genre_ids: Vec<usize>,
    pub id: usize,
    pub original_title: String,
    pub original_language: String,
    pub title: String,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: usize,
    pub video: bool,
    pub vote_average: f64,
}