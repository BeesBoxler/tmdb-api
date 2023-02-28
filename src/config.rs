#[derive(Debug)]
pub struct Config {
    pub endpoint: &'static str,
    pub version: u8,
}

pub const CONFIG: Config = Config {
    endpoint: "https://api.themoviedb.org/",
    version: 3,
};