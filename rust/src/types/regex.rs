use std::{fmt::Display, ops::Deref};

use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [regex](https://typst.app/docs/reference/foundations/regex/)
#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone, Debug, Default, Hash)]
pub struct Regex<'a>(#[serde(borrow)] pub String<'a>);

crate::impl_all!(Item<'a>::Regex, Regex<'a>{'a}, "regex");

impl<'a> Deref for Regex<'a> {
    type Target = String<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Display for Regex<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}/", **self)
    }
}

impl<'a> From<Regex<'a>> for String<'a> {
    fn from(val: Regex<'a>) -> Self {
        val.0
    }
}

impl<'a> From<String<'a>> for Regex<'a> {
    fn from(value: String<'a>) -> Self {
        Regex(value)
    }
}

impl<'a> From<&'a str> for Regex<'a> {
    fn from(value: &'a str) -> Self {
        Regex(String::from(value))
    }
}
