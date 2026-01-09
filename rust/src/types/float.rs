use std::fmt::Display;

use crate::types::{Item, r#type::TypeName};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum Float{
    // f32(f32),
    f64(f64),
}

impl Into<f64> for Float {
    fn into(self) -> f64 {
        match self {
            // Float::f32(f) => f as f64,
            Float::f64(f) => f,
        }
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Float::f32(val) => val.fmt(f),
            Float::f64(val) => val.fmt(f),
        }
    }
}

impl<'a> TryFrom<Item<'a>> for Float {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Float(fl) => Ok(fl),
            _ => Err(format!("Invalid type for Float: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Float {
    fn into(self) -> Item<'a> {
        Item::Float(self)
    }
}

impl<'a> TypeName for Float {
    fn name() -> &'static str {
        "float"
    }
}