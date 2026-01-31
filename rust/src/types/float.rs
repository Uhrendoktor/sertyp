use std::fmt::Display;

use crate::Item;

/// For more information visit the typst documentation: [float](https://typst.app/docs/reference/foundations/float/)
/// # Note
/// typst only supports 64-bit floats. It correctly deserializes 32-bit floats from cbor, but then reinterprets them as 64-bit floats, leading to wrong values.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum Float {
    // f32(f32),
    f64(f64),
}

crate::impl_all!(Item<'a>::Float, Float {}, "float");

impl Default for Float {
    fn default() -> Self {
        Float::f64(0.0)
    }
}

impl From<Float> for f64 {
    fn from(val: Float) -> Self {
        match val {
            // Float::f32(f) => f as f64,
            Float::f64(f) => f,
        }
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float::f64(value)
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
