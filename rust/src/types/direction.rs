use derive_more::{Display, IsVariant};

use crate::Item;

/// For more information visit the typst documentation: [direction](https://typst.app/docs/reference/layout/direction/)
#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    Hash,
    IsVariant,
    Display,
)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    #[display("ltr")]
    LTR,
    #[display("rtl")]
    RTL,
    #[display("ttb")]
    TTB,
    #[display("btt")]
    BTT,
}

crate::impl_all!(Item<'a>::Direction, Direction {}, "direction");
