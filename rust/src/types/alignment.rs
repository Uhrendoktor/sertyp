use crate::Item;

/// For more information visit the typst documentation: [alignment](https://typst.app/docs/reference/layout/alignment/)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Start,
    End,
    #[default]
    Left,
    Center,
    Right,
    Top,
    Horizon,
    Bottom,
}

crate::impl_all!(Item<'a>::Alignment, Alignment {}, "alignment");
