use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub enum AreaType {
    Country,
    Subdivision,
    County,
    Municipality,
    City,
    District,
    Island
}

impl fmt::Display for AreaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AreaType::Country => write!(f, "Country"),
            AreaType::Subdivision => write!(f, "Subdivision"),
            AreaType::County => write!(f, "County"),
            AreaType::Municipality => write!(f, "Municipality"),
            AreaType::City => write!(f, "City"),
            AreaType::District => write!(f, "District"),
            AreaType::Island => write!(f, "Island")
        }
    }
}
