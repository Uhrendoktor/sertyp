use std::{fmt::Display, hash::Hash};

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

impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            // Float::f32(f) => f.to_bits().hash(state),
            Float::f64(f) => f.to_bits().hash(state),
        }
    }
}

impl Default for Float {
    fn default() -> Self {
        Float::f64(0.0)
    }
}

macro_rules! impl_into {
    ($($t:ty),*) => {
        $(
            impl From<Float> for $t {
                fn from(val: Float) -> Self {
                    match val {
                        // Float::f32(f) => f as $t,
                        Float::f64(f) => f as $t,
                    }
                }
            }
        )*
    };
}
impl_into!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Float {
                fn from(value: $t) -> Self {
                    Float::f64(value as f64)
                }
            }
        )*
    };
}
impl_from!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Float::f32(val) => val.fmt(f),
            Float::f64(val) => val.fmt(f),
        }
    }
}
