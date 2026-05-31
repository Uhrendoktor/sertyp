use std::fmt::Debug;

use derive_more::{Deref, DerefMut, From, Into, IntoIterator};

use crate::{Array, Item, TypstTypeLike};

/// A typed version of [Array]. Supports any type that implements [TryFrom]/[Into] of [Item].
/// # Example
/// ```
/// use sertyp::{typst_func, TypedArray, Integer, String};
/// //#[typst_func]
/// fn sum_array<'a>(arr: TypedArray<Integer>) -> Result<Integer, String<'a>> {
///     Ok(arr.into_iter()
///         .map(|i| {
///             i.try_into()
///                 .map_err(|e| format!("Failed to convert Integer to i32: {e}").into())
///         }).collect::<Result<Vec<i32>, String<'a>>>()?
///         .into_iter()
///         .sum::<i32>()
///         .into())
/// }
/// ```
#[derive(Clone, Debug, Default, Hash, IntoIterator, Deref, DerefMut, From, Into)]
#[into_iterator(owned, ref, ref_mut)]
#[from(Vec<T>)]
#[into(Vec<T>)]
pub struct TypedArray<T>(pub Vec<T>);

impl<T> TypedArray<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<'a, 'de: 'a, T: TryFrom<Item<'a>>> serde::Deserialize<'de> for TypedArray<T>
where
    T::Error: std::fmt::Display,
{
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

impl<'a, T: TryFrom<Item<'a>>> TryFrom<Array<'a>> for TypedArray<T> {
    type Error = T::Error;

    fn try_from(value: Array<'a>) -> std::result::Result<Self, Self::Error> {
        Ok(value
            .into_iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<_>, _>>()?
            .into())
    }
}

impl<'a, T: Into<Item<'a>>> From<TypedArray<T>> for Array<'a> {
    fn from(val: TypedArray<T>) -> Self {
        let arr: Array = val
            .0
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<_>>()
            .into();
        arr
    }
}

impl<'a, T: TypstTypeLike + TryFrom<Item<'a>>> TryFrom<Item<'a>> for TypedArray<T>
where
    <T as TryFrom<Item<'a>>>::Error: Into<std::string::String>,
{
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        match value {
            Item::Array(arr) => Ok(arr.try_into().map_err(Into::into)?),
            other => Err(format!(
                "Type was expected to be {}, found {:?}",
                <Self as TypstTypeLike>::static_type_name(),
                other
            )),
        }
    }
}

impl<'a, T: Into<Item<'a>>> From<TypedArray<T>> for Item<'a> {
    fn from(val: TypedArray<T>) -> Self {
        let arr: Array = val.into();
        Item::Array(arr)
    }
}

impl<T: TypstTypeLike> TypstTypeLike for TypedArray<T> {
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("array<{}>", T::static_type_name()))
    }
}
