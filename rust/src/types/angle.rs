use derive_more::{Display, IsVariant};

use crate::{Item, types::float::Float};

/// For more information visit the typst documentation: [angle](https://typst.app/docs/reference/layout/angle/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Display)]
#[display("{}{}", value, unit)]
pub struct Angle {
    pub value: Float,
    pub unit: AngleUnit,
}

crate::impl_all!(Item<'a>::Angle, Angle {}, "angle");

/// Unit of angle. Typst automatically casts into `deg` even if another unit was specified.
/// The `rad` variant can still be used when constructing values in rust.
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
pub enum AngleUnit {
    #[serde(rename = "rad")]
    #[display("rad")]
    Radians,
    #[default]
    #[serde(rename = "deg")]
    #[display("deg")]
    Degrees,
}
