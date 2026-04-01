use crate::{Content, RBox, TypedItem};

/// For more information visit the typst documentation: [math.attach](https://typst.app/docs/reference/math/attach/)

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Attach<'a> {
    #[serde(borrow)]
    pub base: TypedItem<RBox<Content<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub t: Option<TypedItem<RBox<Content<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub b: Option<TypedItem<RBox<Content<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub tl: Option<TypedItem<RBox<Content<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub bl: Option<TypedItem<RBox<Content<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub tr: Option<TypedItem<RBox<Content<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub br: Option<TypedItem<RBox<Content<'a>>>>,
}

crate::impl_all!(Content<'a>::MathAttach, Attach<'a>{'a}, "math.attach");
