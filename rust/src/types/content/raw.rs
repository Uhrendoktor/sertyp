use crate::{Alignment, Array, AutoOr, Boolean, Bytes, Integer, Or, String, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Raw<'a> {
    #[serde(borrow)]
    pub text: TypedItem<String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<TypedItem<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub lang: Option<TypedItem<String<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<TypedItem<Alignment>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub syntaxes: Option<RawSyntaxes<'a>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub theme: Option<AutoOr<Or<String<'a>, Bytes<'a>>>>,
    #[serde(rename="tab-size", skip_serializing_if = "Option::is_none")]
    pub tab_size: Option<TypedItem<Integer>>,
}

crate::impl_all_content!(Raw<'a>, "raw");

crate::auto_impl!{
    #[derive(Clone, Debug)]
    pub enum RawSyntaxes<'a> {
        String(String<'a>),
        Bytes(Bytes<'a>),
        Array(Array<'a>),
    }
}