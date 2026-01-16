use crate::{Alignment, AutoOr, Boolean, Box, Content, Function, Or, String, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Equation<'a> {
    pub block: Boolean,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub numbering: Option<Or<String<'a>, Function<'a>>>,
    #[serde(rename = "number-align", skip_serializing_if = "Option::is_none")]
    pub number_align: Option<Box<Alignment>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub supplement: Option<AutoOr<Or<Box<TypedItem<Content<'a>>>, Function<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String<'a>>,
    pub body: Box<TypedItem<Content<'a>>>
}

crate::impl_all_content!(Equation<'a>.Math, "math.class");