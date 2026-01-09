use crate::types::{Item, r#type::TypeName};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    LTR,
    RTL,
    TTB,
    BTT
}

impl<'a> TryFrom<Item<'a>> for Direction {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Direction(d) => Ok(d),
            _ => Err(format!("Invalid type for Direction: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Direction {
    fn into(self) -> Item<'a> {
        Item::Direction(self)
    }
}

impl<'a> TypeName for Direction {
    fn name() -> &'static str {
        "direction"
    }
}