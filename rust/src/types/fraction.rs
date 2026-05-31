use derive_more::{Display, IsVariant};

use crate::{Item, types::float::Float};

/// For more information visit the typst documentation: [fraction](https://typst.app/docs/reference/layout/fraction/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default, Display)]
#[display("{}{}", value, unit)]
pub struct Fraction {
    pub value: Float,
    pub unit: FractionUnit,
}

crate::impl_all!(Item<'a>::Fraction, Fraction {}, "fraction");

#[derive(
    Default,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Display,
    IsVariant,
)]
pub enum FractionUnit {
    #[default]
    #[serde(rename = "fr")]
    #[display("fr")]
    Fraction,
}
