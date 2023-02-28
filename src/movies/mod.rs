pub mod alternative_titles;
pub mod changes;
pub mod credits;
pub mod movie;
pub mod status;
pub mod external_ids;
pub mod images;
pub mod keywords;

pub use alternative_titles::AlternativeTitles;
pub use changes::Changes;
pub use credits::Credits;
pub use movie::Movie;
pub use status::Status;
pub use external_ids::ExternalIds;
pub use images::Images;
pub use keywords::Keywords;

use reqwest::{Response, Result};

pub struct Movies {
    api_key: String,
}

impl Movies {
    pub fn create(api_key: &str) -> Self {
        Movies {
            api_key: String::from(api_key),
        }
    }

    async fn get(&self, path: &str, id: usize) -> Result<Response> {
        let url = super::url::build_url(format!("/movie/{id}{path}"), &self.api_key);
        reqwest::get(url).await
    }

    pub async fn get_movie(&self, id: usize) -> Result<Movie> {
        self.get("", id).await?.json().await
    }

    pub async fn get_alternative_titles(&self, id: usize) -> Result<AlternativeTitles> {
        self.get("/alternative_titles", id).await?.json().await
    }

    pub async fn get_changes(&self, id: usize) -> Result<Changes> {
        self.get("/changes", id).await?.json().await
    }

    pub async fn get_credits(&self, id: usize) -> Result<Credits> {
        self.get("/credits", id).await?.json().await
    }

    pub async fn get_external_ids(&self, id: usize) -> Result<ExternalIds> {
        self.get("/external_ids", id).await?.json().await
    }

    pub async fn get_images(&self, id: usize) -> Result<Images> {
        self.get("/images", id).await?.json().await
    }
}
