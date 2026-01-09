use std::fmt::Display;

use crate::types::{float::Float, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Length {
    #[serde(rename = "value")]
    pub abs: Float,
    pub unit: AbsUnit,
    pub em: Float,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AbsUnit {
    #[serde(rename = "pt")]
    Points,
    #[serde(rename = "in")]
    Inches,
    #[serde(rename = "cm")]
    Centimeters,
    #[serde(rename = "mm")]
    Millimeters,
}

impl Display for AbsUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsUnit::Points => write!(f, "pt"),
            AbsUnit::Inches => write!(f, "in"),
            AbsUnit::Centimeters => write!(f, "cm"),
            AbsUnit::Millimeters => write!(f, "mm"),
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} + {}em", self.abs, self.unit, self.em)
    }
}

impl<'a> TryFrom<crate::types::Item<'a>> for Length {
    type Error = std::string::String;

    fn try_from(value: crate::types::Item<'a>) -> Result<Self, Self::Error> {
        match value {
            crate::types::Item::Length(l) => Ok(l),
            _ => Err(format!("Invalid type for Length: {:?}", value)),
        }
    }
}

impl<'a> Into<crate::types::Item<'a>> for Length {
    fn into(self) -> crate::types::Item<'a> {
        crate::types::Item::Length(self)
    }
}

impl<'a> TypeName for Length {
    fn name() -> &'static str {
        "length"
    }
}