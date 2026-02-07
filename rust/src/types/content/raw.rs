use crate::{Alignment, Array, AutoOr, Boolean, Bytes, Content, Integer, Or, String, TypedItem};

/// For more information visit the typst documentation: [raw](https://typst.app/docs/reference/text/raw/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
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
    #[serde(rename = "tab-size", skip_serializing_if = "Option::is_none")]
    pub tab_size: Option<TypedItem<Integer>>,
}

crate::impl_all!(Content<'a>::Raw, Raw<'a>{'a}, "raw");

crate::auto_impl! {
    #[derive(Clone, Debug, Hash)]
    pub enum RawSyntaxes<'a> {
        try_from{},
        String(String=>String<'a>),
        Bytes(Bytes=>Bytes<'a>),
        Array(Array=>Array<'a>),
    }
}
