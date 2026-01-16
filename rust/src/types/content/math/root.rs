use crate::{Content, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Root<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub index: Option<Box<TypedItem<Content<'a>>>>,
    #[serde(borrow)]
    pub radicand: Box<TypedItem<Content<'a>>>,
}

crate::impl_all_content!(Root<'a>.Math, "math.root");