use derive_more::{Display, From, IsVariant, TryInto, TryUnwrap, Unwrap};

use crate::Item;

/// For more information visit the typst documentation: [integer](https://typst.app/docs/reference/foundations/int/)
#[derive(
    Copy,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    Hash,
    IsVariant,
    Unwrap,
    TryUnwrap,
    Display,
    From,
    TryInto,
)]
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
