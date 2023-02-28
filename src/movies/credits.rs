use serde::Deserialize;
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Credits {
    pub id: usize,
    pub cast: Vec<CastMember>,
    pub crew: Vec<CrewMember>,
}

#[derive(Deserialize, Debug, Eq)]
pub struct CastMember {
    pub adult: bool,
    pub gender: Option<usize>,
    pub id: usize,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    // pub popularity: usize, // needs to be float
    pub profile_path: Option<String>,
    pub cast_id: usize,
    pub character: String,
    pub credit_id: String,
    pub order: usize,
}

impl PartialEq for CastMember { 
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Deserialize, Debug, Eq)]
pub struct CrewMember {
    pub adult: bool,
    pub gender: Option<usize>,
    pub id: usize,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    // pub popularity: usize, // needs to be float
    pub profile_path: Option<String>,
    pub credit_id: String,
    pub department: String,
    pub job: String,
}

impl PartialEq for CrewMember { 
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}