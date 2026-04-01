use crate::{AutoOr, Content, Length, RBox, Stroke, TypedItem, types::boolean::Boolean};

/// For more information visit the typst documentation: [underline](https://typst.app/docs/reference/math/underover/#functions-underline)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Underline<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<TypedItem<Boolean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evade: Option<TypedItem<Boolean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<TypedItem<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<AutoOr<Length>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<AutoOr<Stroke<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub body: Option<RBox<TypedItem<Content<'a>>>>,
}

crate::impl_all!(Content<'a>::Underline, Underline<'a> {'a}, "underline");
