use std::ops::Deref;

use crate::types::{Item, string::String, r#type::TypeName};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Symbol<'a>(
    #[serde(borrow)]
    String<'a>
);

impl<'a> Deref for Symbol<'a> {
    type Target = String<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> TryFrom<Item<'a>> for Symbol<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Symbol(s) => Ok(s),
            _ => Err(format!("Invalid type for Symbol: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Symbol<'a> {
    fn into(self) -> Item<'a> {
        Item::Symbol(self)
    }
}

impl<'a> TypeName for Symbol<'a> {
    fn name() -> &'static str {
        "symbol"
    }
}