use crate::{Content, Box};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Frac<'a> {
    #[serde(borrow)]
    pub num: Box<Content<'a>>,
    #[serde(borrow)]
    pub denom: Box<Content<'a>>,
    pub style: FracStyle,
}

crate::impl_all_content!(Frac<'a>.Math, "math.frac");

crate::auto_impl_str!(
    pub enum FracStyle {
        Vertical = "vertical",
        Skewed = "skewed",
        Horizontal = "horizontal",
    }
);