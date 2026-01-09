use std::ops::Deref;

use crate::types::{Item, string::String};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Type<'a>(
    #[serde(borrow)]
    String<'a>
);

impl<'a> Deref for Type<'a> {
    type Target = String<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> TryFrom<Item<'a>> for Type<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Type(a) => Ok(a),
            _ => Err(format!("Invalid type for Type: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Type<'a> {
    fn into(self) -> Item<'a> {
        Item::Type(self)
    }
}

pub trait TypeName {
    fn name() -> &'static str;

    fn type_name(&self) -> &'static str {
        Self::name()
    }
}

impl<'a> TypeName for Type<'a> {
    fn name() -> &'static str {
        "type"
    }
}