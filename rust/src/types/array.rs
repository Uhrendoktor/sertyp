use std::{fmt::Debug, ops::{Deref, DerefMut}};
use crate::types::{Item, Item_};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Array_<'a>(
    #[serde(borrow)]
    pub Vec<Item_<'a>>
);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(from = "Array_", into = "Array_")]
pub struct Array<'a>(
    #[serde(borrow)]
    Vec<Item<'a>>
);

crate::impl_all!(Array<'a>, "array");

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

impl<'a> Deref for Array<'a> {
    type Target = Vec<Item<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Array<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> IntoIterator for Array<'a> {
    type Item = Item<'a>;
    type IntoIter = std::vec::IntoIter<Item<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> From<Vec<Item<'a>>> for Array<'a> {
    fn from(value: Vec<Item<'a>>) -> Self {
        Array(value)
    }
}

#[derive(Clone, Debug)]
pub struct Pair<T> (T, T);

impl<'a, T: Clone> TryFrom<Array<'a>> for Pair<T>
where
    T: TryFrom<Item<'a>, Error=std::string::String>,
{
    type Error = std::string::String;

    fn try_from(value: Array<'a>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(format!("Expected array of length 2 for Pair, got length {}", value.len()));
        }
        let t1 = T::try_from(value[0].clone())
            .map_err(|e| format!("Error converting first element of Pair: {}", e))?;
        let t2 = T::try_from(value[1].clone())
            .map_err(|e| format!("Error converting second element of Pair: {}", e))?;
        Ok(Pair(t1, t2))
    }
}

impl<'a, T: Clone + Into<Item<'a>>> Into<Array<'a>> for Pair<T> 
{
    fn into(self) -> Array<'a> {
        Array(vec![self.0.into(), self.1.into()])
    }
}

impl<'a, T> TryFrom<Item<'a>> for Pair<T>
where
    Pair<T>: TryFrom<Array<'a>, Error=std::string::String>,
{
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Array(arr) => Pair::try_from(arr),
            other => Err(format!("Tried to convert Item to Pair, found {:?}", other)),
        }
    }
}

impl<'a, T: Clone + Into<Item<'a>>> Into<Item<'a>> for Pair<T> {
    fn into(self) -> Item<'a> {
        let array: Array<'a> = Pair(self.0.clone(), self.1.clone()).into();
        Item::Array(array)
    }
}

impl<'a, 'de: 'a, T> serde::Deserialize<'de> for Pair<T>
where
    Pair<T>: TryFrom<Array<'a>, Error=std::string::String>,
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