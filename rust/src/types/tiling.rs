use crate::{Length, types::array::Pair};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tiling{
    pub size: Pair<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Pair<Length>>
}

crate::impl_all!(Tiling, "tiling");