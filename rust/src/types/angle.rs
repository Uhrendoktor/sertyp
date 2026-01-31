use std::fmt::Display;

use crate::{Item, types::float::Float};

/// For more information visit the typst documentation: [angle](https://typst.app/docs/reference/layout/angle/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Angle {
    pub value: Float,
    pub unit: AngleUnit,
}

crate::impl_all!(Item<'a>::Angle, Angle {}, "angle");

/// Unit of angle. Typst automatically casts into `deg` even if another unit was specified.
/// The `rad` variant can still be used when constructing values in rust.
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AngleUnit {
    #[serde(rename = "rad")]
    Radians,
    #[default]
    #[serde(rename = "deg")]
    Degrees,
}

impl Display for AngleUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AngleUnit::Radians => write!(f, "rad"),
            AngleUnit::Degrees => write!(f, "deg"),
        }
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}
