use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Keywords {
    pub id: usize,
    pub keywords: Vec<Keyword>,
}

#[derive(Debug,Deserialize)]
pub struct Keyword {
    pub id: usize,
    pub name: String
}