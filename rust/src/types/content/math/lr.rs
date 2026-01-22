use crate::{Box, Content, Relative, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LR<'a> {
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<TypedItem<Relative>>,
    #[serde(borrow)]
    pub body: Box<Content<'a>>
}

crate::impl_all_content!(LR<'a>.Math, "math.lr");
