use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub enum DistanceUnit {
    Centimeters,
    Meters,
    Kilometers,
}

impl FromStr for DistanceUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CENTIMERES" | "CM" => Ok(Self::Centimeters),
            "METERS" | "M" => Ok(Self::Meters),
            "KILOMETERS" | "KM" => Ok(Self::Kilometers),
            _ => Err("Unknown unit"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Latitude(pub f32);

#[derive(Debug, PartialEq)]
pub struct Longitude(pub f32);

#[derive(Debug, Error, PartialEq)]
pub enum LatLngError {
    #[error("Latitude must be between -90° and 90°")]
    LatitudeOutOfRange,
    #[error("Longitude must be between -180° and 180°")]
    LongitudeOutOfRange,
}

impl TryFrom<f32> for Longitude {
    type Error = LatLngError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        match value {
            val if value >= -180.0 && val <= 180.0 => Ok(Longitude(val)),
            _ => Err(LatLngError::LongitudeOutOfRange),
        }
    }
}

pub trait TrySubstract<RHS = Self> {
    type Output;
    type Err;

    fn try_substract(self, rhs: RHS) -> Result<Self::Output, Self::Err>;
}

impl TrySubstract for Longitude {
    type Output = Self;

    type Err = LatLngError;

    fn try_substract(self, rhs: Self) -> Result<Self::Output, Self::Err> {
        let sum = self.0 - rhs.0;
        if !(-180.0..=180.0).contains(&sum) {
            return Err(LatLngError::LongitudeOutOfRange);
        }

        Ok(Self(sum))
    }
}

impl TrySubstract for Latitude {
    type Output = Self;

    type Err = LatLngError;

    fn try_substract(self, rhs: Self) -> Result<Self::Output, Self::Err> {
        let sum = self.0 - rhs.0;
        if !(-90.0..=90.0).contains(&sum) {
            return Err(LatLngError::LatitudeOutOfRange);
        }

        Ok(Self(sum))
    }
}

pub trait TryAdd<RHS = Self> {
    type Output;
    type Err;

    fn try_add(self, rhs: RHS) -> Result<Self::Output, Self::Err>;
}

impl TryAdd for Longitude {
    type Output = Self;

    type Err = LatLngError;

    fn try_add(self, rhs: Self) -> Result<Self::Output, Self::Err> {
        let sum = self.0 + rhs.0;
        if !(-180.0..=180.0).contains(&sum) {
            return Err(LatLngError::LongitudeOutOfRange);
        }

        Ok(Self(sum))
    }
}

impl TryAdd for Latitude {
    type Output = Self;

    type Err = LatLngError;

    fn try_add(self, rhs: Self) -> Result<Self::Output, Self::Err> {
        let sum = self.0 + rhs.0;
        if !(-90.0..=90.0).contains(&sum) {
            return Err(LatLngError::LatitudeOutOfRange);
        }

        Ok(Self(sum))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_add() {
        assert_eq!(
            Longitude(20.0).try_add(Longitude(-30.0)),
            Ok(Longitude(-10.0))
        );
        assert_eq!(
            Longitude(-180.0).try_add(Longitude(0.0)),
            Ok(Longitude(-180.0))
        );
        assert_eq!(
            Longitude(-180.0).try_add(Longitude(0.0)),
            Ok(Longitude(-180.0))
        );

        assert_eq!(
            Longitude(-180.0).try_add(Longitude(-1.0)),
            Err(LatLngError::LongitudeOutOfRange)
        );

        assert_eq!(
            Longitude(90.0).try_add(Longitude(200.1)),
            Err(LatLngError::LongitudeOutOfRange)
        );
    }

    #[test]
    fn test_try_subtract() {
        assert_eq!(
            Latitude(90.0).try_substract(Latitude(50.0)),
            Ok(Latitude(40.0))
        );

        assert_eq!(
            Latitude(-89.1).try_substract(Latitude(12.33)),
            Err(LatLngError::LatitudeOutOfRange)
        )
    }
}
