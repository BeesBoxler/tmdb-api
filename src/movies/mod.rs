pub mod alternative_titles;
pub mod changes;
pub mod credits;
pub mod external_ids;
pub mod images;
pub mod keywords;
pub mod lists;
pub mod movie;
pub mod movie_results;
pub mod reviews;
pub mod status;

pub use alternative_titles::AlternativeTitles;
pub use changes::Changes;
pub use credits::Credits;
pub use external_ids::ExternalIds;
pub use images::Images;
pub use keywords::Keywords;
pub use lists::Lists;
pub use movie::Movie;
pub use reviews::Reviews;
pub use status::Status;

use reqwest::{Response, Result};
use movie_results::{DatedMovieResults, MovieResults};


pub struct Movies {
    api_key: String,
}

impl Movies {
    pub fn create(api_key: &str) -> Self {
        Movies {
            api_key: String::from(api_key),
        }
    }

    async fn get(&self, path: &str) -> Result<Response> {
        let url = super::url::build_url(format!("/movie/{path}"), &self.api_key);
        reqwest::get(url).await
    }

    pub async fn get_movie(&self, id: usize) -> Result<Movie> {
        self.get(&format!("{id}")).await?.json().await
    }

    pub async fn get_alternative_titles(&self, id: usize) -> Result<AlternativeTitles> {
        self.get(&format!("{id}/alternative_titles")).await?.json().await
    }

    pub async fn get_changes(&self, id: usize) -> Result<Changes> {
        self.get(&format!("/{id}/changes")).await?.json().await
    }

    pub async fn get_credits(&self, id: usize) -> Result<Credits> {
        self.get(&format!("/{id}/credits")).await?.json().await
    }

    pub async fn get_external_ids(&self, id: usize) -> Result<ExternalIds> {
        self.get(&format!("/{id}/external_ids")).await?.json().await
    }

    pub async fn get_images(&self, id: usize) -> Result<Images> {
        self.get(&format!("/{id}/images")).await?.json().await
    }

    pub async fn get_lists(&self, id: usize) -> Result<Lists> {
        self.get(&format!("/{id}/lists")).await?.json().await
    }

    pub async fn get_recommendations(&self, id: usize) -> Result<MovieResults> {
        self.get(&format!("/{id}/recommendations")).await?.json().await
    }

    pub async fn get_reviews(&self, id: usize) -> Result<Reviews> {
        self.get(&format!("/{id}/reviews")).await?.json().await
    }

    pub async fn get_similar_movies(&self, id: usize) -> Result<MovieResults> {
        self.get(&format!("/{id}/similar")).await?.json().await
    }

    pub async fn get_latest(&self) -> Result<Movie> {
        self.get("/latest").await?.json().await
    }

    pub async fn get_now_playing(&self) -> Result<DatedMovieResults> {
        self.get("/now_playing").await?.json().await
    }

    pub async fn get_popular(&self) -> Result<MovieResults> {
        self.get("/popular").await?.json().await
    }

    pub async fn get_top_rated(&self) -> Result<MovieResults> {
        self.get("/top_rated").await?.json().await
    }

    pub async fn get_upcoming(&self) -> Result<DatedMovieResults> {
        self.get("/upcoming").await?.json().await
    }
}
