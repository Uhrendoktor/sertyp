use crate::{Box, Content, TypedItem};

/// For more information visit the typst documentation: [math.root](https://typst.app/docs/reference/math/roots/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Root<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub index: Option<TypedItem<Box<Content<'a>>>>,
    #[serde(borrow)]
    pub radicand: TypedItem<Box<Content<'a>>>,
}

crate::impl_all!(Content<'a>::MathRoot, Root<'a>{'a}, "math.root");
