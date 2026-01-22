use std::fmt::Display;

use crate::types::float::Float;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Ratio {
    pub value: Float,
    pub unit: RatioUnit
}

crate::impl_all!(Ratio, "ratio");

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RatioUnit {
    #[serde(rename = "%")]
    Percent,
}

impl Default for RatioUnit {
    fn default() -> Self {
        RatioUnit::Percent
    }
}

impl Display for RatioUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RatioUnit::Percent => write!(f, "%"),
        }
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}