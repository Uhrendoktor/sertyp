use crate::{Box, Content, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Frac<'a> {
    #[serde(borrow)]
    pub num: Box<Content<'a>>,
    #[serde(borrow)]
    pub denom: Box<Content<'a>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub style: Option<TypedItem<FracStyle>>,
}

crate::impl_all_content!(Frac<'a>.Math, "math.frac");

crate::auto_impl_str!(
    pub enum FracStyle {
        Vertical = "vertical",
        Skewed = "skewed",
        Horizontal = "horizontal",
    }
);