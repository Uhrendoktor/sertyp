use std::collections::HashMap;
use crate::types::{Item, Item_, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Dictionary_<'a>(
    #[serde(borrow)]
    HashMap<&'a str, Item_<'a>>
);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(from = "Dictionary_", into = "Dictionary_")]
pub struct Dictionary<'a>(
    #[serde(borrow)]
    HashMap<&'a str, Item<'a>>
);

impl<'a> From<Dictionary<'a>> for Dictionary_<'a> {
    fn from(value: Dictionary<'a>) -> Self {
        Dictionary_(value.0.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<'a> From<Dictionary_<'a>> for Dictionary<'a> {
    fn from(value: Dictionary_<'a>) -> Self {
        Dictionary(value.0.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<'a> std::ops::Deref for Dictionary<'a> {
    type Target = HashMap<&'a str, Item<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for Dictionary<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryFrom<Item<'a>> for Dictionary<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Dictionary(d) => Ok(d),
            _ => Err(format!("Invalid type for Dictionary: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Dictionary<'a> {
    fn into(self) -> Item<'a> {
        Item::Dictionary(self)
    }
}

impl<'a> TypeName for Dictionary<'a> {
    fn name() -> &'static str {
        "dictionary"
    }
}