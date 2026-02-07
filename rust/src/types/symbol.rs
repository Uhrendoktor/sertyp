use std::{borrow::Cow, fmt::Display, ops::Deref};

use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [symbol](https://typst.app/docs/reference/foundations/symbol/)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default, Hash)]
pub struct Symbol<'a>(#[serde(borrow)] pub Cow<'a, char>);

impl<'a> Deref for Symbol<'a> {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<String<'a>> for Symbol<'a> {
    fn from(value: String<'a>) -> Self {
        Symbol(Cow::Owned(value.chars().next().unwrap_or_default()))
    }
}

impl<'a> From<Symbol<'a>> for String<'a> {
    fn from(value: Symbol<'a>) -> Self {
        String::from(value.0.to_string())
    }
}

impl<'a> From<Symbol<'a>> for std::string::String {
    fn from(value: Symbol<'a>) -> Self {
        value.0.to_string()
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

impl Display for Symbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

crate::impl_all!(Item<'a>::Symbol, Symbol<'a>{'a}, "symbol");
