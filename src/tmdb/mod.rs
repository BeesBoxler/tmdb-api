mod config;
mod movie;
mod person;

use reqwest::{self, Response, Result};
pub use movie::{Credits, Movie};
pub use person::Person;

pub struct Tmdb {
    api_key: String,
}

impl Tmdb {
    pub fn create(api_key: &str) -> Self {
        Tmdb {api_key: String::from(api_key)}
    }

    pub async fn get_movie(&self, id: &str) -> Result<Movie> {
        self.get(format!("/movie/{id}")).await?.json::<Movie>().await
    }

    pub async fn get_credits(&self, id: &str) -> Result<Credits> {
        self.get(format!("/movie/{id}/credits")).await?.json::<Credits>().await
    }

    async fn get(&self, path: String) -> Result<Response> {
        let config::Config { endpoint, version } = config::CONFIG;
        let api_key = &*self.api_key;
        let url = format!("{endpoint}{version}{path}?api_key={api_key}");

        reqwest::get(url).await
    }
}
