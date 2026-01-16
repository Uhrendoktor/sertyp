use crate::{Alignment, Array, AutoOr, Boolean, Bytes, Integer, Or, String};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Raw<'a> {
    #[serde(borrow)]
    pub text: String<'a>,
    pub block: Boolean,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Alignment>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub syntaxes: Option<RawSyntaxes<'a>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub theme: Option<AutoOr<Or<String<'a>, Bytes<'a>>>>,
    #[serde(rename="tab-size", skip_serializing_if = "Option::is_none")]
    pub tab_size: Option<Integer>,
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