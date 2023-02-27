use serde::Deserialize;

#[derive(Deserialize, Debug, Hash, Eq)]
pub struct Person {
    pub name: String,
    pub id: usize,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}