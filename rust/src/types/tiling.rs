use crate::{Item, Length, types::array::Pair};

/// For more information visit the typst documentation: [tiling](https://typst.app/docs/reference/visualize/tiling/)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Hash)]
pub struct Tiling {
    pub size: Pair<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Pair<Length>>,
}

crate::impl_all!(Item<'a>::Tiling, Tiling {}, "tiling");
