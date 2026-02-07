use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{Item, TypstTypeLike};

/// A `std::boxed::Box` wrapper that implements `TryFrom<Item>` and `Into<Item>` and therefore supports most of sertyp's internal serialization/deserialization magic.
/// Unless you write your own type variants, you probably won't need this.
#[derive(Clone, Debug, Default, Hash)]
pub struct Box<T>(pub std::boxed::Box<T>);
impl<T> Box<T> {
    pub fn into_inner(self) -> T {
        *self.0
    }
}
impl<'a, 'de: 'a, T> Deserialize<'de> for Box<T>
where
    Self: 'a,
    Box<T>: TryFrom<Item<'a>>,
    <Box<T> as TryFrom<Item<'a>>>::Error: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item = Item::deserialize(deserializer)?;
        Box::try_from(item).map_err(serde::de::Error::custom)
    }
}

impl<'a, T: Clone + Into<Item<'a>>> Serialize for Box<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let item: Item = (**self).clone().into();
        item.serialize(serializer)
    }
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Box<T> {
    fn from(value: T) -> Self {
        Box(std::boxed::Box::new(value))
    }
}

impl<'a, T: Clone + Into<Item<'a>>> From<Box<T>> for Item<'a> {
    fn from(val: Box<T>) -> Self {
        let value: T = (*val).clone();
        value.into()
    }
}

impl<'a, T: TryFrom<Item<'a>, Error = std::string::String>> TryFrom<Item<'a>> for Box<T> {
    type Error = T::Error;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        T::try_from(value.clone()).map(|t| t.into())
    }
}

impl<T: TypstTypeLike> TypstTypeLike for Box<T> {
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        T::static_type_name()
    }
}
