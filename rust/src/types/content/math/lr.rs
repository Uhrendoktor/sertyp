use crate::{Box, Content, Relative};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LR<'a> {
    pub size: Relative,
    #[serde(borrow)]
    pub body: Box<Content<'a>>
}

crate::impl_all_content!(LR<'a>.Math, "math.lr");
