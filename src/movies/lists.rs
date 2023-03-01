use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Lists {
    pub id: usize,
    pub page: usize,
    pub results: Vec<List>,
    pub total_pages: usize,
    pub total_results: usize,
}

#[derive(Debug, Deserialize)]
pub struct List {
    pub description: String,
    pub favourite_count: usize,
    pub id: usize,
    pub item_count: usize,
    pub iso_639_1: String,
    pub list_type: String,
    pub name: String,
    pub poster_path: Option<String>,
}