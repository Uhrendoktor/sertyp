use crate::{Box, Content, Relative};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LR<'a> {
    #[serde(borrow)]
    pub size: Relative<'a>,
    #[serde(borrow)]
    pub body: Box<Content<'a>>
}

crate::impl_all_content!(LR<'a>.Math, "math.lr");
