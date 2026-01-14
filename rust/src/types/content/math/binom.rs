use crate::{Array, Box, Content};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Binom<'a> {
    #[serde(borrow)]
    pub upper: Box<Content<'a>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub lower: Option<Box<Array<'a>>>,
}

crate::impl_all_content!(Binom<'a>.Math, "math.binom");