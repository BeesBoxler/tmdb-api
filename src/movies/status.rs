use serde::{Deserialize, de::Error};

#[derive(Debug)]
pub enum Status {
    Rumoured,
    Planned,
    InProduction,
    PostProduction,
    Released,
    Canceled,
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let status = String::deserialize(deserializer)?.to_lowercase();
        let result = match status.as_str() {
            "rumoured" => Status::Rumoured,
            "planned" => Status::Planned,
            "in production" => Status::InProduction,
            "post production" => Status::PostProduction,
            "released" => Status::Released,
            "canceled" => Status::Canceled,
            e => return Err(Error::custom(format!("'{e}' is not a valid status.")))
        };

        Ok(result)
    }
}