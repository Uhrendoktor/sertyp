use crate::{Boolean, Content, TypedItem, Box};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Op<'a> {
    #[serde(borrow)]
    pub text: Box<Content<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<TypedItem<Boolean>>
}

crate::impl_all_content!(Op<'a>.Math, "math.op");