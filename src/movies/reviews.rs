use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Reviews {
    pub id: usize,
    pub page: usize,
    pub results: Vec<Review>,
    pub total_pages: usize,
    pub total_results: usize,
}

#[derive(Deserialize, Debug)]
pub struct Review {
    pub author: String,
    pub author_details: ReviewAuthor,
    pub content: String,
    pub created_at: String,
    pub id: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ReviewAuthor {
    pub name: String,
    pub username: String,
    pub avatar_path: Option<String>,
    pub rating: Option<usize>,
}
