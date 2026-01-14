use crate::{Content, String};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Class<'a> {
    #[serde(borrow)]
    pub class: String<'a>,
    pub body: Box<Content<'a>>,
}

crate::impl_all_content!(Class<'a>.Math, "math.class");