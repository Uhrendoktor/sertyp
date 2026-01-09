use std::fmt::Display;

use crate::types::{Item, float::Float, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Fraction {
    pub value: Float,
    pub unit: FractionUnit
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FractionUnit {
    #[serde(rename = "fr")]
    Fraction,
}

impl Display for FractionUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FractionUnit::Fraction => write!(f, "fr"),
        }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

impl<'a> TryFrom<Item<'a>> for Fraction {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Fraction(fr) => Ok(fr),
            _ => Err(format!("Invalid type for Fraction: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Fraction {
    fn into(self) -> Item<'a> {
        Item::Fraction(self)
    }
}

impl<'a> TypeName for Fraction {
    fn name() -> &'static str {
        "fraction"
    }
}