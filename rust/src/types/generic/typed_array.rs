use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

use crate::{Array, Item, TypstTypeLike};

#[derive(Clone, Debug, Default)]
pub struct TypedArray<T>(pub Vec<T>);

impl<'a, 'de: 'a, T: TryFrom<Item<'a>, Error=std::string::String>> serde::Deserialize<'de> for TypedArray<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item: Item = Item::deserialize(deserializer)?;
        let arr: Array = item.try_into().map_err(serde::de::Error::custom)?;
        arr.try_into().map_err(serde::de::Error::custom)
    }
}

impl<'a, T: Clone + Into<Item<'a>>> serde::Serialize for TypedArray<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let arr: Array = self.clone().into();
        arr.serialize(serializer)
    }
}

impl<T> Deref for TypedArray<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for TypedArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for TypedArray<T> {
    fn from(value: Vec<T>) -> Self {
        TypedArray(value)
    }
}

impl<
    'a,
    T: TryFrom<Item<'a>, Error=std::string::String>,
> TryFrom<Array<'a>> for TypedArray<T> {
    type Error = std::string::String;

    fn try_from(value: Array<'a>) -> std::result::Result<Self, Self::Error> {
        Ok(value.into_iter().map(|v| v.try_into()).collect::<Result<Vec<_>, _>>()?.into())
    }
}

impl<'a, T: Into<Item<'a>>> Into<Array<'a>> for TypedArray<T> {
    fn into(self) -> Array<'a> {
        let arr: Array = self.0.into_iter().map(|v| v.into()).collect::<Vec<_>>().into();
        arr.into()
    }
}

impl<'a, T: TypstTypeLike + TryFrom<Item<'a>, Error=std::string::String>> TryFrom<Item<'a>> for TypedArray<T> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        match value {
            Item::Array(arr) => Ok(arr.try_into()?),
            other => Err(format!("Type was expected to be {}, found {:?}", <Self as TypstTypeLike>::static_type_name(), other).into()),
        }
    }
}

impl<'a, T: Into<Item<'a>>> Into<Item<'a>> for TypedArray<T> {
    fn into(self) -> Item<'a> {
        let arr: Array = self.into();
        Item::Array(arr)
    }
}

impl<T: TypstTypeLike> TypstTypeLike for TypedArray<T> {
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("array<{}>", T::static_type_name()))
    }
}
