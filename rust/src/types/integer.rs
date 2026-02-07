use std::{fmt::Display, num::TryFromIntError};

use crate::Item;

/// For more information visit the typst documentation: [integer](https://typst.app/docs/reference/foundations/int/)
#[derive(Copy, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum Integer {
    i8(i8),
    i16(i16),
    i32(i32),
    i64(i64),
    i128(i128),
    u8(u8),
    u16(u16),
    u32(u32),
    u64(u64),
    u128(u128),
    isize(isize),
    usize(usize),
}

impl Default for Integer {
    fn default() -> Self {
        Integer::i32(0)
    }
}

crate::impl_all!(Item<'a>::Integer, Integer {}, "integer");

#[derive(Debug)]
pub enum IntegerError {
    Convert(TryFromIntError),
}

impl From<TryFromIntError> for IntegerError {
    fn from(err: TryFromIntError) -> Self {
        IntegerError::Convert(err)
    }
}

impl From<std::convert::Infallible> for IntegerError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

macro_rules! impl_integer_conversions {
    ($($v:ident),*) => {
        $(
            impl From<$v> for Integer {
                fn from(value: $v) -> Self {
                    Integer::$v(value)
                }
            }

            impl TryFrom<Integer> for $v {
                type Error = IntegerError;
                fn try_from(value: Integer) -> Result<Self, Self::Error> {
                    Ok(match value {
                        Integer::i8(v) => v.try_into()?,
                        Integer::i16(v) => v.try_into()?,
                        Integer::i32(v) => v.try_into()?,
                        Integer::i64(v) => v.try_into()?,
                        Integer::i128(v) => v.try_into()?,
                        Integer::u8(v) => v.try_into()?,
                        Integer::u16(v) => v.try_into()?,
                        Integer::u32(v) => v.try_into()?,
                        Integer::u64(v) => v.try_into()?,
                        Integer::u128(v) => v.try_into()?,
                        Integer::isize(v) => v.try_into()?,
                        Integer::usize(v) => v.try_into()?,
                    })
                }
            }
        )*
    };
}

impl_integer_conversions!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize
);

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::i8(val) => val.fmt(f),
            Integer::i16(val) => val.fmt(f),
            Integer::i32(val) => val.fmt(f),
            Integer::i64(val) => val.fmt(f),
            Integer::i128(val) => val.fmt(f),
            Integer::u8(val) => val.fmt(f),
            Integer::u16(val) => val.fmt(f),
            Integer::u32(val) => val.fmt(f),
            Integer::u64(val) => val.fmt(f),
            Integer::u128(val) => val.fmt(f),
            Integer::isize(val) => val.fmt(f),
            Integer::usize(val) => val.fmt(f),
        }
    }
}
