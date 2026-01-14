#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Start,
    End,
    Left,
    Center,
    Right,
    Top,
    Horizon,
    Bottom,
}

crate::impl_all!(Alignment, "alignment");