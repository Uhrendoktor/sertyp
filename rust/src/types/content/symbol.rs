use crate::{Content, String, Symbol};

/// A content version of [Symbol]. See the [Symbol] type for more information.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Symbol_<'a> {
    #[serde(borrow)]
    pub text: String<'a>,
}

impl<'a> From<Symbol_<'a>> for Symbol<'a> {
    fn from(val: Symbol_<'a>) -> Self {
        val.text.into()
    }
}

impl<'a> From<Symbol<'a>> for Symbol_<'a> {
    fn from(value: Symbol<'a>) -> Self {
        Symbol_ { text: value.0 }
    }
}

crate::impl_try_from!(Content<'a>::Symbol, Symbol<'a>);
crate::impl_into!(Content<'a>::Symbol, Symbol<'a>);
