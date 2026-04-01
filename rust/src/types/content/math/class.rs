use crate::{Content, RBox, TypedItem};

/// For more information visit the typst documentation: [math.class](https://typst.app/docs/reference/math/class/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Class<'a> {
    pub class: TypedItem<ClassVariant>,
    #[serde(borrow)]
    pub body: TypedItem<RBox<Content<'a>>>,
}

crate::auto_impl_str! {
    #[derive(Default)]
    pub enum ClassVariant{
        #[default]
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

crate::impl_all!(Content<'a>::MathClass, Class<'a>{'a}, "math.class");
