use std::{fmt::Display, num::TryFromIntError};

use crate::types::{Item, r#type::TypeName};



#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl TryInto<i32> for Integer {
    type Error = TryFromIntError;
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Integer::i8(i) => Ok(i as i32),
            Integer::i16(i) => Ok(i as i32),
            Integer::i32(i) => Ok(i),
            Integer::i64(i) => i32::try_from(i),
            Integer::i128(i) => i32::try_from(i),
            Integer::u8(u) => Ok(u as i32),
            Integer::u16(u) => Ok(u as i32),
            Integer::u32(u) => i32::try_from(u),
            Integer::u64(u) => i32::try_from(u),
            Integer::u128(u) => i32::try_from(u),
            Integer::isize(i) => i32::try_from(i),
            Integer::usize(u) => i32::try_from(u),
        }
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Integer::i32(value)
    }
}

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

impl<'a> TryFrom<Item<'a>> for Integer {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Integer(i) => Ok(i),
            _ => Err(format!("Invalid type for Integer: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Integer {
    fn into(self) -> Item<'a> {
        Item::Integer(self)
    }
}

impl<'a> TypeName for Integer {
    fn name() -> &'static str {
        "integer"
    }
}