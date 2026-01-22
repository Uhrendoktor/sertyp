use crate::{Box, Content};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Attach<'a> {
    #[serde(borrow)]
    pub base: Box<Content<'a>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub t: Option<Box<Content<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub b: Option<Box<Content<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub tl: Option<Box<Content<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub bl: Option<Box<Content<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub tr: Option<Box<Content<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub br: Option<Box<Content<'a>>>,
}

crate::impl_all_content!(Attach<'a>.Math, "math.attach");