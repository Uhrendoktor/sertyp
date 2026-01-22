use std::{fmt::Display, ops::Deref};

use crate::types::string::String;

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct Regex<'a> (
    #[serde(borrow)]
    pub String<'a>,
);

crate::impl_all!(Regex<'a>, "regex");

impl<'a> Deref for Regex<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Display for Regex<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}/", &*self)
    }
}

impl<'a> Into<String<'a>> for Regex<'a> {
    fn into(self) -> String<'a> {
        self.0
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