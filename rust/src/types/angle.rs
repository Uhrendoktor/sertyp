use std::fmt::Display;

use crate::types::float::Float;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Angle {
    pub value: Float,
    pub unit: AngleUnit
}

crate::impl_all!(Angle, "angle");

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AngleUnit {
    #[serde(rename = "rad")]
    Radians,
    #[serde(rename = "deg")]
    Degrees
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