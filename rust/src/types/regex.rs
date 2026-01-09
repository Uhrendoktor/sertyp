use std::{fmt::Display, ops::Deref};

use crate::types::{Item, string::String, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Regex<'a> (
    #[serde(borrow)]
    String<'a>,
);

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

impl<'a> TryFrom<Item<'a>> for Regex<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Regex(r) => Ok(r),
            _ => Err(format!("Invalid type for Regex: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Regex<'a> {
    fn into(self) -> Item<'a> {
        Item::Regex(self)
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

impl<'a> TypeName for Regex<'a> {
    fn name() -> &'static str {
        "regex"
    }
}