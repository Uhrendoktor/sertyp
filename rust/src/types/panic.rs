use crate::types::{Item, r#type::TypeName, string::String};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Panic<'a> (
    #[serde(borrow)]
    &'a str
);

impl<'a> TryFrom<Item<'a>> for Panic<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Panic(p) => Ok(p),
            _ => Err(format!("Invalid type for Panic: {:?}", value)),
        }    
    }
}

impl<'a> From<&'a str> for Panic<'a> {
    fn from(value: &'a str) -> Self {
        Panic(value)
    }
}

impl<'a> From<String<'a>> for Panic<'a> {
    fn from(value: String<'a>) -> Self {
        Panic(value.0)
    }
}

impl<'a> Into<Item<'a>> for Panic<'a> {
    fn into(self) -> Item<'a> {
        Item::Panic(self)
    }
}

impl<'a> TypeName for Panic<'a> {
    fn name() -> &'static str {
        "panic"
    }
}