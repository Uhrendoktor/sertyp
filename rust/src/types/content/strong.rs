use crate::{Content, Integer, Box, TypedItem, impl_all_content};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Strong<'a> {
    #[serde(skip_serializing_if="Option::is_none")]
    pub delta: Option<TypedItem<Integer>>,
    #[serde(borrow)]
    pub body: Box<TypedItem<Content<'a>>>
}

impl_all_content!(Strong<'a>, "strong");
