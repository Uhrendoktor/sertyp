use derive_more::{Deref, DerefMut, From, Into, IntoIterator};

use crate::types::{Item, Item_};
use std::fmt::Debug;

/// Pre-deserialization / post-serialization helper struct for [Array]. You probably want to use [Array] instead.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct Array_<'a>(#[serde(borrow)] pub Vec<Item_<'a>>);

/// For more information visit the typst documentation: [array](https://typst.app/docs/reference/foundations/array/)
/// # Note
/// For homogenous known types, consider using [crate::TypedArray] instead.
///
/// # Example
/// Constructing an `Array` of integers and converting it into an `Item`:
///
/// ```rust
/// use sertyp::{Array, Item, Integer};
/// let arr: Array = vec![Integer::i32(1).into(), Integer::i32(2).into()].into();
/// let item: Item = arr.into();
/// ```
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Debug,
    Default,
    Deref,
    DerefMut,
    IntoIterator,
    From,
    Into,
)]
#[serde(from = "Array_", into = "Array_")]
#[into_iterator(owned, ref, ref_mut)]
pub struct Array<'a>(#[serde(borrow)] Vec<Item<'a>>);

crate::impl_all!(Item<'a>::Array, Array<'a>{'a}, "array");

impl<'a> From<Array<'a>> for Array_<'a> {
    fn from(value: Array<'a>) -> Self {
        Array_(value.0.into_iter().map(|x| x.into()).collect())
    }
}

impl<'a> From<Array_<'a>> for Array<'a> {
    fn from(value: Array_<'a>) -> Self {
        Array(value.0.into_iter().map(|x| x.into()).collect())
    }
}

#[derive(Clone, Debug, Default, Hash)]
pub struct Pair<T>(pub T, pub T);

impl<'a, T: Clone + TryFrom<Item<'a>>> TryFrom<Array<'a>> for Pair<T>
where
    T::Error: std::fmt::Display,
{
    type Error = std::string::String;

    fn try_from(value: Array<'a>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(format!(
                "Expected array of length 2 for Pair, got length {}",
                value.len()
            ));
        }
        let t1 = T::try_from(value[0].clone())
            .map_err(|e| format!("Error converting first element of Pair: {e}"))?;
        let t2 = T::try_from(value[1].clone())
            .map_err(|e| format!("Error converting second element of Pair: {e}"))?;
        Ok(Pair(t1, t2))
    }
}

impl<'a, T: Clone + Into<Item<'a>>> From<Pair<T>> for Array<'a> {
    fn from(val: Pair<T>) -> Self {
        Array(vec![val.0.into(), val.1.into()])
    }
}

impl<'a, T> TryFrom<Item<'a>> for Pair<T>
where
    Pair<T>: TryFrom<Array<'a>>,
    <Pair<T> as TryFrom<Array<'a>>>::Error: Into<std::string::String>,
{
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Array(arr) => Pair::try_from(arr).map_err(Into::into),
            other => Err(format!("Tried to convert Item to Pair, found {other:?}")),
        }
    }
}

impl<'a, T: Clone + Into<Item<'a>>> From<Pair<T>> for Item<'a> {
    fn from(val: Pair<T>) -> Self {
        let array: Array<'a> = Pair(val.0.clone(), val.1.clone()).into();
        Item::Array(array)
    }
}

impl<'a, 'de: 'a, T> serde::Deserialize<'de> for Pair<T>
where
    Pair<T>: TryFrom<Array<'a>>,
    <Pair<T> as TryFrom<Array<'a>>>::Error: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array = Array::deserialize(deserializer)?;
        Pair::try_from(array).map_err(serde::de::Error::custom)
    }
}

impl<'a, T: Clone + Into<Item<'a>>> serde::Serialize for Pair<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let array: Array<'a> = Pair(self.0.clone(), self.1.clone()).into();
        array.serialize(serializer)
    }
}
