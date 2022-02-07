use super::units::{Latitude, Longitude};

#[derive(Debug)]
pub struct City {
    name: String,
    latitude: Option<Latitude>,
    longitude: Option<Longitude>,
}

impl City {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            latitude: None,
            longitude: None,
        }
    }
}
