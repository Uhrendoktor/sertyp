use std::{fmt::Display, ops::Deref};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct String<'a> (
    #[serde(borrow)]
    pub std::borrow::Cow<'a, str>
);
crate::impl_all!(String<'a>, "string");

impl Deref for String<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a str> for String<'a> {
    fn from(value: &'a str) -> Self {
        String(std::borrow::Cow::Borrowed(value))
    }
}

impl<'a> From<std::borrow::Cow<'a, str>> for String<'a> {
    fn from(value: std::borrow::Cow<'a, str>) -> Self {
        String(value)
    }
}

impl<'a> From<std::string::String> for String<'a> {
    fn from(value: std::string::String) -> Self {
        String(std::borrow::Cow::Owned(value))
    }
}

impl<'a> Display for String<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}