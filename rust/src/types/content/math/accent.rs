use crate::{Boolean, Box, Content, Or, Relative, String, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Accent<'a> {
    #[serde(borrow)]
    pub base: Box<TypedItem<Content<'a>>>,
    pub accent: Or<String<'a>, Box<Content<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<TypedItem<Relative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dotless: Option<TypedItem<Boolean>>,
}

crate::impl_all_content!(Accent<'a>.Math, "math.accent");