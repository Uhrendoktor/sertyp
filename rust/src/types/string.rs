use std::{fmt::Display, ops::Deref};

use crate::types::{Item, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct String<'a> (
    #[serde(borrow)]
    pub(crate) &'a str,
);

impl Deref for String<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a str> for String<'a> {
    fn from(value: &'a str) -> Self {
        String(value)
    }
}

impl<'a> Display for String<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> TryFrom<Item<'a>> for String<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::String(s) => Ok(s),
            _ => Err(format!("Invalid type for String: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for String<'a> {
    fn into(self) -> Item<'a> {
        Item::String(self)
    }
}

impl<'a> TypeName for String<'a> {
    fn name() -> &'static str {
        "string"
    }
}