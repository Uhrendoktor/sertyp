use crate::Content;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Class<'a> {
    pub class: ClassVariant,
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

crate::impl_all_content!(Class<'a>.Math, "math.class");