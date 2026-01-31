use std::fmt::Display;

use crate::{Item, types::float::Float};

/// For more information visit the typst documentation: [fraction](https://typst.app/docs/reference/layout/fraction/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Fraction {
    pub value: Float,
    pub unit: FractionUnit,
}

crate::impl_all!(Item<'a>::Fraction, Fraction {}, "fraction");

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FractionUnit {
    #[default]
    #[serde(rename = "fr")]
    Fraction,
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
