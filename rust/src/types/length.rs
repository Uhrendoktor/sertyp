use std::fmt::Display;

use crate::types::float::Float;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Length {
    #[serde(rename = "value")]
    pub abs: Float,
    pub unit: AbsUnit,
    pub em: Float,
}

crate::impl_all!(Length, "length");

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AbsUnit {
    #[serde(rename = "pt")]
    Points,
    #[serde(rename = "in")]
    Inches,
    #[serde(rename = "cm")]
    Centimeters,
    #[serde(rename = "mm")]
    Millimeters,
}

impl Default for AbsUnit {
    fn default() -> Self {
        AbsUnit::Points
    }
}

impl Display for AbsUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsUnit::Points => write!(f, "pt"),
            AbsUnit::Inches => write!(f, "in"),
            AbsUnit::Centimeters => write!(f, "cm"),
            AbsUnit::Millimeters => write!(f, "mm"),
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} + {}em", self.abs, self.unit, self.em)
    }
}