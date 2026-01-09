use std::fmt::Display;

use crate::types::{Item, float::Float, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Ratio {
    pub value: Float,
    pub unit: RatioUnit
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RatioUnit {
    #[serde(rename = "%")]
    Percent,
}

impl Display for RatioUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RatioUnit::Percent => write!(f, "%"),
        }
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

impl<'a> TryFrom<Item<'a>> for Ratio {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Ratio(r) => Ok(r),
            _ => Err(format!("Invalid type for Ratio: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Ratio {
    fn into(self) -> Item<'a> {
        Item::Ratio(self)
    }
}

impl<'a> TypeName for Ratio {
    fn name() -> &'static str {
        "ratio"
    }
}