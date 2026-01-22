use crate::{Content, Box};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Root<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub index: Option<Box<Content<'a>>>,
    #[serde(borrow)]
    pub radicand: Box<Content<'a>>,
}

crate::impl_all_content!(Root<'a>.Math, "math.root");