use derive_more::{Display, IsVariant, TryUnwrap, Unwrap};

use crate::Item;

/// For more information visit the typst documentation: [float](https://typst.app/docs/reference/foundations/float/)
/// # Note
/// typst only supports 64-bit floats. It correctly deserializes 32-bit floats from cbor, but then reinterprets them as 64-bit floats (in the typst runtime), leading to wrong values.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    IsVariant,
    Unwrap,
    TryUnwrap,
    Display,
)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
#[unwrap(owned, ref, ref_mut)]
#[try_unwrap(owned, ref, ref_mut)]
pub enum Float {
    // currently not supported by typst runtime. The values can be send and deserialized in typst, but are then interpreted as 64bit.
    // f32(f32),
    f64(f64),
}

crate::impl_all!(Item<'a>::Float, Float {}, "float");

impl Default for Float {
    fn default() -> Self {
        Float::f64(0.0)
    }
}
impl Float {
    pub fn to_f64(&mut self) -> &mut f64 {
        match self {
            // Float::f32(f) => {
            //     let f64_val = *f as f64;
            //     *self = Float::f64(f64_val);
            //     self.unwrap_f_64_mut()
            // }
            Float::f64(f) => f,
        }
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
impl_into!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, f32, f64
);

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
impl_from!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, f32, f64
);
