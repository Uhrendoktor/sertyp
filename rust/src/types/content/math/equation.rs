use crate::{Alignment, AutoOr, Boolean, Box, Content, Function, Or, String, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Equation<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<TypedItem<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub numbering: Option<Or<String<'a>, Function<'a>>>,
    #[serde(rename = "number-align", skip_serializing_if = "Option::is_none")]
    pub number_align: Option<TypedItem<Alignment>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub supplement: Option<AutoOr<Or<Box<Content<'a>>, Function<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub alt: Option<TypedItem<String<'a>>>,
    pub body: Box<Content<'a>>
}

crate::impl_all_content!(Equation<'a>.Math, "math.equation");