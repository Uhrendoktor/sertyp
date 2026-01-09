use std::fmt::Display;

use crate::{Integer, types::{Item, r#type::TypeName}};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Version {
    major: Integer,
    minor: Integer,
    patch: Integer
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl<'a> TryFrom<Item<'a>> for Version {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Version(v) => Ok(v),
            _ => Err(format!("Invalid type for Version: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Version {
    fn into(self) -> Item<'a> {
        Item::Version(self)
    }
}

impl<'a> TypeName for Version {
    fn name() -> &'static str {
        "version"
    }
}