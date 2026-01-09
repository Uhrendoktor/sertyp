use std::fmt::Display;

use crate::types::{float::Float, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Angle {
    pub value: Float,
    pub unit: AngleUnit
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AngleUnit {
    #[serde(rename = "rad")]
    Radians,
    #[serde(rename = "deg")]
    Degrees
}

impl Display for AngleUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AngleUnit::Radians => write!(f, "rad"),
            AngleUnit::Degrees => write!(f, "deg"),
        }
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

impl<'a> TryFrom<crate::types::Item<'a>> for Angle {
    type Error = std::string::String;

    fn try_from(value: crate::types::Item<'a>) -> Result<Self, Self::Error> {
        match value {
            crate::types::Item::Angle(a) => Ok(a),
            _ => Err(format!("Invalid type for Angle: {:?}", value)),
        }
    }
}

impl<'a> Into<crate::types::Item<'a>> for Angle {
    fn into(self) -> crate::types::Item<'a> {
        crate::types::Item::Angle(self)
    }
}

impl<'a> TypeName for Angle {
    fn name() -> &'static str {
        "angle"
    }
}