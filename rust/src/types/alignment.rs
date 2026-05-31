use derive_more::Display;

use crate::Item;

/// For more information visit the typst documentation: [alignment](https://typst.app/docs/reference/layout/alignment/)
#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash, Display,
)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    #[display("start")]
    Start,
    #[display("end")]
    End,
    #[default]
    #[display("left")]
    Left,
    #[display("center")]
    Center,
    #[display("right")]
    Right,
    #[display("top")]
    Top,
    #[display("horizon")]
    Horizon,
    #[display("bottom")]
    Bottom,
}

crate::impl_all!(Item<'a>::Alignment, Alignment {}, "alignment");
