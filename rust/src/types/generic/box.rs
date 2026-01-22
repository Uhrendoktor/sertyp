use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{Item, TypstTypeLike};

#[derive(Clone, Debug, Default)]
pub struct Box<T>(pub std::boxed::Box<T>);
impl<'a, 'de: 'a, T> Deserialize<'de> for Box<T>
where
    Self: 'a,
    Box<T>: TryFrom<Item<'a>, Error=std::string::String>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item = Item::deserialize(deserializer)?;
        Box::try_from(item).map_err(serde::de::Error::custom)
    }
}

impl<'a, T: Clone + Into<Item<'a>>> Serialize for Box<T>
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let item: Item = (&**self).clone().into();
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

impl<T> From<std::boxed::Box<T>> for Box<T> {
    fn from(value: std::boxed::Box<T>) -> Self {
        Box(value)
    }
}

impl<'a, T: Clone + Into<Item<'a>>> Into<Item<'a>> for Box<T> {
    fn into(self) -> Item<'a> {
        let value: T = (&*self).clone();
        value.into()
    }
}

impl<
    'a,
    T: TryFrom<Item<'a>, Error=std::string::String>,
> TryFrom<Item<'a>> for Box<T> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        T::try_from(value.clone()).map(|t| Box(std::boxed::Box::new(t)))
    }
}

impl<T: TypstTypeLike> TypstTypeLike for Box<T> {
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        T::static_type_name()
    }
}
