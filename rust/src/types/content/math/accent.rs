use crate::{Boolean, Box, Content, Or, Relative, String};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Accent<'a> {
    #[serde(borrow)]
    pub base: Box<Content<'a>>,
    pub accent: Or<String<'a>, Box<Content<'a>>>,
    #[serde(borrow)]
    pub size: Relative<'a>,
    pub dotless: Boolean,
}

crate::impl_all_content!(Accent<'a>.Math, "math.accent");