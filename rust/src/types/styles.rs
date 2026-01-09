use std::ops::{Deref, DerefMut};

use crate::types::{Item, r#type::TypeName};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Styles<'a>(
    #[serde(borrow)]
    Box<Item<'a>>
);

impl<'a> Deref for Styles<'a> {
    type Target = Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Styles<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryFrom<Item<'a>> for Styles<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Styles(s) => Ok(s),
            _ => Err(format!("Invalid type for Styles: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Styles<'a> {
    fn into(self) -> Item<'a> {
        Item::Styles(self)
    }
}

impl<'a> TypeName for Styles<'a> {
    fn name() -> &'static str {
        "styles"
    }
}