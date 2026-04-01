use crate::{
    AutoOr, Boolean, Content, Dictionary, Fraction, NoneOr, Or, RBox, Relative, TypedItem,
    types::generic::{FillColor, StrokeColor},
};

/// For more information visit the typst documentation: [box](https://typst.app/docs/reference/layout/box/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Box<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<AutoOr<Or<Relative, Fraction>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<AutoOr<Relative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TypedItem<Relative>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub fill: Option<NoneOr<FillColor<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<NoneOr<StrokeColor<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub radius: Option<Or<Relative, Dictionary<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub inset: Option<Or<Relative, Dictionary<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub outset: Option<Or<Relative, Dictionary<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<TypedItem<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub body: Option<RBox<TypedItem<Content<'a>>>>,
}

crate::impl_all!(Content<'a>::Box, std::boxed::Box<Box<'a>>{'a}, "box");

impl<'a> From<Box<'a>> for Content<'a> {
    fn from(val: Box<'a>) -> Self {
        Content::Box(std::boxed::Box::new(val))
    }
}
