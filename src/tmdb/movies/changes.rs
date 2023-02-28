use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Changes {
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Debug)]
pub struct Change {
    pub key: String,
    pub items: Vec<ChangeItem>,
}

#[derive(Deserialize, Debug)]
pub struct ChangeItem {
    pub id: String,
    pub action: String,
    pub time: String,
    pub iso_639_1: String,
    pub value: String,
    pub original_value: String,
}