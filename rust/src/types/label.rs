use std::fmt::Display;

use crate::types::{Item, string::String, r#type::TypeName};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Label<'a>(
    #[serde(borrow)]
    String<'a>
);

impl<'a> Display for Label<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}

impl<'a> TryFrom<Item<'a>> for Label<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Label(l) => Ok(l),
            _ => Err(format!("Invalid type for Label: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Label<'a> {
    fn into(self) -> Item<'a> {
        Item::Label(self)
    }
}

impl<'a> TypeName for Label<'a> {
    fn name() -> &'static str {
        "label"
    }
}