use std::borrow::Cow;

use derive_more::{Deref, DerefMut, Display, From};

use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [symbol](https://typst.app/docs/reference/foundations/symbol/)
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Deref,
    DerefMut,
    Default,
    Hash,
    Display,
    From,
)]
#[display("{}", **self)]
#[deref(forward)]
#[deref_mut(forward)]
pub struct Symbol<'a>(#[serde(borrow)] pub Cow<'a, char>);

impl<'a> From<String<'a>> for Symbol<'a> {
    fn from(value: String<'a>) -> Self {
        Symbol(Cow::Owned(value.chars().next().unwrap_or_default()))
    }
}

impl<'a> From<&'a char> for Symbol<'a> {
    fn from(value: &'a char) -> Self {
        Symbol(Cow::Borrowed(value))
    }
}

impl<'a> From<char> for Symbol<'a> {
    fn from(value: char) -> Self {
        Symbol(Cow::Owned(value))
    }
}

impl<'a> From<std::string::String> for Symbol<'a> {
    fn from(value: std::string::String) -> Self {
        Symbol(Cow::Owned(value.chars().next().unwrap_or_default()))
    }
}

impl<'a> From<Symbol<'a>> for String<'a> {
    fn from(value: Symbol<'a>) -> Self {
        String::from(std::string::String::from(value))
    }
}

impl<'a> From<Symbol<'a>> for std::string::String {
    fn from(value: Symbol<'a>) -> Self {
        value.0.to_string()
    }
}

crate::impl_all!(Item<'a>::Symbol, Symbol<'a>{'a}, "symbol");
