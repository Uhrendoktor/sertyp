use crate::{Content, Symbol};

/// A content version of [Symbol]. See the [Symbol] type for more information.
///
/// # Note:
/// This struct is purely used for parsing since the typst version of a content symbol has a named field (`text`).
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash)]
pub(crate) struct Symbol_<'a> {
    #[serde(borrow)]
    pub text: std::borrow::Cow<'a, char>,
}

impl<'a> From<Symbol_<'a>> for Symbol<'a> {
    fn from(val: Symbol_<'a>) -> Self {
        Symbol(val.text)
    }
}

impl<'a> From<Symbol<'a>> for Symbol_<'a> {
    fn from(value: Symbol<'a>) -> Self {
        Symbol_ { text: value.0 }
    }
}

crate::impl_try_from!(Content<'a>::Symbol, Symbol<'a>);
crate::impl_into!(Content<'a>::Symbol, Symbol<'a>);
