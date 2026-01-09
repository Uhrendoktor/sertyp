use std::ops::{Deref, DerefMut};

use crate::types::{Item, Item_, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Array_<'a>(
    #[serde(borrow)]
    Vec<Item_<'a>>
);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(from = "Array_", into = "Array_")]
pub struct Array<'a>(
    #[serde(borrow)]
    Vec<Item<'a>>
);

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

impl<'a> From<Vec<Item<'a>>> for Array<'a> {
    fn from(value: Vec<Item<'a>>) -> Self {
        Array(value)
    }
}

impl<'a> TryFrom<Item<'a>> for Array<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Array(a) => Ok(a),
            _ => Err(format!("Invalid type for Array: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Array<'a> {
    fn into(self) -> Item<'a> {
        Item::Array(self)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(try_from = "Array", into = "Array")]
pub struct Pair<'a, T: Clone + Into<Item<'a>> + TryFrom<Item<'a>>> (
    T, T, 
    #[serde(borrow)]
    #[allow(dead_code)]
    &'a ()
)
where
    <T as TryFrom<Item<'a>>>::Error: std::fmt::Display;

impl<'a, T: Clone> TryFrom<Array<'a>> for Pair<'a, T>
where
    T: Into<Item<'a>> + TryFrom<Item<'a>>,
    <T as TryFrom<Item<'a>>>::Error: std::fmt::Display,
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
        Ok(Pair(t1, t2, &()))
    }
}

impl<'a, T: Clone + Into<Item<'a>> + TryFrom<Item<'a>>> Into<Array<'a>> for Pair<'a, T>
where
    <T as TryFrom<Item<'a>>>::Error: std::fmt::Display
{
    fn into(self) -> Array<'a> {
        Array(vec![self.0.into(), self.1.into()])
    }
}

impl<'a> TypeName for Array<'a> {
    fn name() -> &'static str {
        "array"
    }
}