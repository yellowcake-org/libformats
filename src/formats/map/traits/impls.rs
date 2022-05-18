use crate::common::types::errors;
use crate::common::types::geometry::{Coordinate, Elevation, Orientation, Scaled};

impl TryFrom<i32> for Coordinate<u8, std::ops::Range<u8>> {
    type Error = errors::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from(u32::try_from(value).map_err(|_| errors::Error::Format)?)
    }
}

impl TryFrom<u32> for Coordinate<u8, std::ops::Range<u8>> {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const SCALE: std::ops::Range<u8> = 0u8..200;
        if (0..(SCALE.end as u32).pow(2)).contains(&value) {
            let x = value / SCALE.end as u32;
            let y = value - (x * SCALE.end as u32);

            Ok(Self {
                x: Scaled { value: x as u8, scale: SCALE },
                y: Scaled { value: y as u8, scale: SCALE },
            })
        } else {
            return Err(errors::Error::Format);
        }
    }
}

impl TryFrom<u32> for Elevation {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (0..3).contains(&value) {
            Ok(Self { level: Scaled { value: value as u8, scale: u8::MIN..3 } })
        } else {
            return Err(errors::Error::Format);
        }
    }
}

impl TryFrom<u32> for Orientation {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (0..6).contains(&value) {
            Ok(Self { value: Scaled { value: value as u8, scale: 0u8..6 } })
        } else {
            return Err(errors::Error::Format);
        }
    }
}