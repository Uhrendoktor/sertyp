use std::fmt::Display;

use crate::types::{Item, array::Array, string::String, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Module<'a> {
    #[serde(borrow)]
    pub name: String<'a>,
    pub member: Array<'a>,
}

impl<'a> Display for Module<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<module {}>", self.name)
    }
}

impl<'a> TryFrom<Item<'a>> for Module<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Module(m) => Ok(m),
            _ => Err(format!("Invalid type for Module: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Module<'a> {
    fn into(self) -> Item<'a> {
        Item::Module(self)
    }
}

impl<'a> TypeName for Module<'a> {
    fn name() -> &'static str {
        "module"
    }
}
