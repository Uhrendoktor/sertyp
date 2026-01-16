use crate::{Content, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Styled<'a> {
    #[serde(borrow)]
    pub child: Box<TypedItem<Content<'a>>>,
}

crate::impl_all_content!(Styled<'a>.Math, "math.styled");