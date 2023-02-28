use super::config;

pub fn build_url(path: String, api_key: &str) -> String {
    let config::Config { endpoint, version } = config::CONFIG;
    let api_key = &*api_key;
    format!("{endpoint}{version}{path}?api_key={api_key}")
}