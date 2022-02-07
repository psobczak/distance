use num_traits::Signed;
use std::fmt::Display;
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
pub struct Latitude<T: Signed + Display>(pub T);

#[derive(Debug, PartialEq)]
pub struct Longitude<T: Signed + Display>(pub T);

#[derive(Debug, Error, PartialEq)]
pub enum LatLngError {
    #[error("Latitude must be between -90° and 90°")]
    LatitudeOutOfRange,
    #[error("Longitude must be between -180° and 180°")]
    LongitudeOutOfRange,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Notrh,
    East,
    South,
    West,
    Center,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Notrh => write!(f, "N"),
            Direction::East => write!(f, "E"),
            Direction::South => write!(f, "S"),
            Direction::West => write!(f, "W"),
            Direction::Center => write!(f, ""),
        }
    }
}

pub trait Side {
    fn get_direction(&self) -> Direction;
}

macro_rules! impl_side_latitude {
    ($($t:ty),+) => {$(
        impl Side for Latitude<$t> {
            fn get_direction(&self) -> Direction {
                match self.0 {
                    val if val >= -90.0 as $t && val < 0.0 as $t => Direction::South,
                    val if val <= 90.0 as $t && val > 0.0 as $t => Direction::Notrh,
                    _ => Direction::Center,
                }
            }
        }
    )+};
}

impl_side_latitude!(i8, i16, i32, i64, i128, isize, f32, f64);

macro_rules! impl_side_longitude {
    ($($t:ty),+) => {$(
        impl Side for Longitude<$t> {
            fn get_direction(&self) -> Direction {
                match self.0 {
                    val if val >= -180.0 as $t && val < 0.0 as $t => Direction::East,
                    val if val <= 180.0 as $t && val > 0.0 as $t => Direction::West,
                    _ => Direction::Center,
                }
            }
        }
    )+};
}

impl_side_longitude!(i8, i16, i32, i64, i128, isize, f32, f64);

macro_rules! impl_display {
    ( $lat_lng:tt, $($t:ty),+ ) => { $(
        impl Display for $lat_lng<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}°", self.0)
            }
        }
    )+};
}

impl_display!(Latitude, i8, i16, i32, i64, i128, isize, f32, f64);
impl_display!(Longitude, i8, i16, i32, i64, i128, isize, f32, f64);

macro_rules! impl_try_from {
    ( $lat_lng:tt, $err_variant:expr, $max_value:expr ,$($t:ty),+) => { $(
        impl TryFrom<$t> for $lat_lng<$t> {

            type Error = LatLngError;

            fn try_from(value: $t) -> Result<Self, Self::Error> {
                if (-$max_value as isize..=$max_value as isize).contains(&(value as isize)) {
                    return Ok(Self(value));
                }

                Err($err_variant)
            }
        }
    )+};
}

impl_try_from!(
    Latitude,
    LatLngError::LatitudeOutOfRange,
    90,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64
);

impl_try_from!(
    Longitude,
    LatLngError::LongitudeOutOfRange,
    180,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64
);

pub trait TrySubstract<RHS = Self> {
    type Output;
    type Err;

    fn try_substract(self, rhs: RHS) -> Result<Self::Output, Self::Err>;
}

pub trait TryAdd<RHS = Self> {
    type Output;
    type Err;

    fn try_add(self, rhs: RHS) -> Result<Self::Output, Self::Err>;
}

macro_rules! impl_try_substract {
    ( $lat_lng:tt, $err_variant:expr, $max_value:expr ,$($t:ty),+) => { $(
        impl TrySubstract<$t> for $lat_lng<$t> {
            type Output = Self;

            type Err = LatLngError;

            fn try_substract(self, rhs: $t) -> Result<Self::Output, Self::Err> {
                let sum = (self.0 - rhs) as isize;
                if !(-$max_value as isize..=$max_value as isize).contains(&sum) {
                    return Err($err_variant);
                }

                Ok(Self(sum as $t))
            }
        }
    )+};
}

impl_try_substract!(
    Latitude,
    LatLngError::LatitudeOutOfRange,
    90,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64
);

impl_try_substract!(
    Longitude,
    LatLngError::LongitudeOutOfRange,
    180,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64
);

macro_rules! impl_try_add {
    ( $lat_lng:tt, $err_variant:expr, $max_value:expr ,$($t:ty),+) => { $(
        impl TryAdd<$t> for $lat_lng<$t> {
            type Output = Self;

            type Err = LatLngError;

            fn try_add(self, rhs: $t) -> Result<Self::Output, Self::Err> {
                let sum = (self.0 + rhs) as isize;
                if !(-$max_value as isize..=$max_value as isize).contains(&sum) {
                    return Err($err_variant);
                }

                Ok(Self(sum as $t))
            }
        }
    )+};
}

impl_try_add!(
    Latitude,
    LatLngError::LatitudeOutOfRange,
    90,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64
);

impl_try_add!(
    Longitude,
    LatLngError::LongitudeOutOfRange,
    180,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_add() {
        assert_eq!(Longitude(20.0).try_add(-30.0), Ok(Longitude(-10.0)));
        assert_eq!(Longitude(-180.0).try_add(0.0), Ok(Longitude(-180.0)));
        assert_eq!(Longitude(-180).try_add(0), Ok(Longitude(-180)));

        assert_eq!(
            Longitude(-180.0).try_add(-1.0),
            Err(LatLngError::LongitudeOutOfRange)
        );

        assert_eq!(
            Longitude(90.0).try_add(200.1),
            Err(LatLngError::LongitudeOutOfRange)
        );

        assert_eq!(Latitude(10).try_add(20), Ok(Latitude(30)));
    }

    #[test]
    fn test_try_subtract() {
        assert_eq!(Latitude(90.0).try_substract(50.0), Ok(Latitude(40.0)));

        assert_eq!(
            Latitude(-89.1).try_substract(12.33),
            Err(LatLngError::LatitudeOutOfRange)
        );

        assert_eq!(Latitude(-50).try_substract(-50), Ok(Latitude(0)));

        assert_eq!(
            Latitude(-50).try_substract(-200),
            Err(LatLngError::LatitudeOutOfRange)
        );
    }

    #[test]
    fn test_direction() {
        assert_eq!(Latitude(0).get_direction(), Direction::Center);
        assert_eq!(Latitude(-12.33).get_direction(), Direction::South);
        assert_eq!(Latitude(12.33).get_direction(), Direction::Notrh);
        assert_eq!(Longitude(-32.33).get_direction(), Direction::East);
        assert_eq!(Longitude(32.33).get_direction(), Direction::West);
    }
}
