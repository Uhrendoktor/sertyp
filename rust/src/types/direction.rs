#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    LTR,
    RTL,
    TTB,
    BTT
}

crate::impl_all!(Direction, "direction");