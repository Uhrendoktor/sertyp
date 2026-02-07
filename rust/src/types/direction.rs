use crate::Item;

/// For more information visit the typst documentation: [direction](https://typst.app/docs/reference/layout/direction/)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    LTR,
    RTL,
    TTB,
    BTT,
}

crate::impl_all!(Item<'a>::Direction, Direction {}, "direction");
