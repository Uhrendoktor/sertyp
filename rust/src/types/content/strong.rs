use crate::{Content, Integer, TypedItem};

/// For more information visit the typst documentation: [strong](https://typst.app/docs/reference/model/strong/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Strong<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<TypedItem<Integer>>,
    #[serde(borrow)]
    pub body: TypedItem<Box<Content<'a>>>,
}

crate::impl_all!(Content<'a>::Strong, Strong<'a>{'a}, "strong");
