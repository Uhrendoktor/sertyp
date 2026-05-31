use crate::{Alignment, AutoOr, Boolean, Content, Length, Relative, String, TypedItem};

/// For more information visit the typst documentation: [place](https://typst.app/docs/reference/layout/place/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Place<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<AutoOr<Alignment>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub scope: Option<TypedItem<String<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float: Option<TypedItem<Boolean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearance: Option<TypedItem<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<TypedItem<Relative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<TypedItem<Relative>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<TypedItem<Content<'a>>>>,
}

crate::impl_all!(Content<'a>::Place, Place<'a>{'a}, "place");
