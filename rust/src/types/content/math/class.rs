use crate::{Box, Content, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Class<'a> {
    pub class: TypedItem<ClassVariant>,
    #[serde(borrow)]
    pub body: Box<Content<'a>>,
}

crate::auto_impl_str!{
    pub enum ClassVariant{
        Normal = "normal",
        Punctuation = "punctuation",
        Opening = "opening",
        Closing = "closing",
        Fence = "fence",
        Large = "large",
        Relation = "relation",
        Unary = "unary",
        Binary = "binary",
        Vary = "vary",
    }
}

impl Default for ClassVariant {
    fn default() -> Self {
        ClassVariant::Normal
    }
}

crate::impl_all_content!(Class<'a>.Math, "math.class");