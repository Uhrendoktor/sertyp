use std::fmt::Display;

use crate::types::{Item, array::Array, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Relative<'a>(
    #[serde(borrow)]
    Array<'a>
);

impl Display for Relative<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // join with +
        let s = self.0.iter()
            .map(|item| format!("{:?}", item))
            .collect::<Vec<_>>()
            .join(" + ");
        write!(f, "{}", s)    
    }
}

impl<'a> TryFrom<Item<'a>> for Relative<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Relative(r) => Ok(r),
            _ => Err(format!("Invalid type for Relative: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Relative<'a> {
    fn into(self) -> Item<'a> {
        Item::Relative(self)
    }
}

impl<'a> TypeName for Relative<'a> {
    fn name() -> &'static str {
        "relative"
    }
}