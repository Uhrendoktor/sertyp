use crate::{Content, RBox, TypedItem};

/// For more information visit the typst documentation: [math.root](https://typst.app/docs/reference/math/roots/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Root<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub index: Option<TypedItem<RBox<Content<'a>>>>,
    #[serde(borrow)]
    pub radicand: TypedItem<RBox<Content<'a>>>,
}

crate::impl_all!(Content<'a>::MathRoot, Root<'a>{'a}, "math.root");
