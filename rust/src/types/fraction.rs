use std::fmt::Display;

use crate::types::float::Float;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Fraction {
    pub value: Float,
    pub unit: FractionUnit
}

crate::impl_all!(Fraction, "fraction");

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FractionUnit {
    #[serde(rename = "fr")]
    Fraction,
}

impl Default for FractionUnit {
    fn default() -> Self {
        FractionUnit::Fraction
    }
}

impl Display for FractionUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FractionUnit::Fraction => write!(f, "fr"),
        }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}