use crate::{Boolean, Content};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Op<'a> {
    #[serde(borrow)]
    pub text: Box<Content<'a>>,
    pub limits: Boolean
}

crate::impl_all_content!(Op<'a>.Math, "math.op");