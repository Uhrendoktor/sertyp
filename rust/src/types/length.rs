use std::fmt::Display;

use crate::{Item, types::float::Float};

/// For more information visit the typst documentation: [length](https://typst.app/docs/reference/layout/length/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default, Hash)]
pub struct Length {
    #[serde(rename = "value")]
    pub abs: Float,
    pub unit: AbsUnit,
    pub em: Float,
}

crate::impl_all!(Item<'a>::Length, Length {}, "length");

impl Length {
    pub fn pt(value: impl Into<Float>) -> Self {
        Length {
            abs: value.into(),
            unit: AbsUnit::Points,
            em: 0.0.into(),
        }
    }

    pub fn inches(value: impl Into<Float>) -> Self {
        Length {
            abs: value.into(),
            unit: AbsUnit::Inches,
            em: 0.0.into(),
        }
    }

    pub fn cm(value: impl Into<Float>) -> Self {
        Length {
            abs: value.into(),
            unit: AbsUnit::Centimeters,
            em: 0.0.into(),
        }
    }

    pub fn mm(value: impl Into<Float>) -> Self {
        Length {
            abs: value.into(),
            unit: AbsUnit::Millimeters,
            em: 0.0.into(),
        }
    }
}

/// Unit of length. Typst automatically casts into `pt` even if another unit was specified.
/// The other variants can still be used when constructing values in rust.
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbsUnit {
    #[default]
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
