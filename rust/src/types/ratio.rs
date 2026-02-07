use std::fmt::Display;

use crate::{Item, types::float::Float};

/// For more information visit the typst documentation: [ratio](https://typst.app/docs/reference/layout/ratio/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default, Hash)]
pub struct Ratio {
    pub value: Float,
    pub unit: RatioUnit,
}

crate::impl_all!(Item<'a>::Ratio, Ratio {}, "ratio");

/// Unit of a ratio value. Typst only supports percentages (`%`).
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RatioUnit {
    #[default]
    #[serde(rename = "%")]
    Percent,
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
