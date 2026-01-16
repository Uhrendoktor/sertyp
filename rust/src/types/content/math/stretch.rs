use crate::{Content, Relative, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Stretch<'a> {
    #[serde(borrow)]
    pub body: Box<TypedItem<Content<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Relative>,
}

crate::impl_all_content!(Stretch<'a>.Math, "math.stretch");