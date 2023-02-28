mod config;
mod url;
pub mod people;
pub mod movies;

pub use movies::Movie;
pub use people::Person;

use self::movies::Movies;

pub struct Tmdb {
    pub movies: Movies
}

impl Tmdb {
    pub fn create(api_key: &str) -> Self {
        Tmdb {
            movies: Movies::create(api_key),
        }
    }
}
