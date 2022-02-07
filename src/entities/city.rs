use std::fmt::Display;

use crate::entities::units::Side;

use super::units::{Latitude, Longitude};

#[derive(Debug)]
pub struct City {
    name: String,
    latitude: Option<Latitude<f32>>,
    longitude: Option<Longitude<f32>>,
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

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} {}, {} {}",
            self.name,
            self.latitude.as_ref().unwrap(),
            self.latitude.as_ref().unwrap().get_direction(),
            self.longitude.as_ref().unwrap(),
            self.longitude.as_ref().unwrap().get_direction()
        )
    }
}
