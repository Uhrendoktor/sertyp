use crate::types::{Item, dictionary::Dictionary, function::Function, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Content<'a> {
    #[serde(borrow)]
    func: Function<'a>,
    fields: Dictionary<'a>
}

impl<'a> TryFrom<Item<'a>> for Content<'a> {
    type Error = &'static str;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Content(c) => Ok(c),
            _ => Err("Invalid type for Content"),
        }
    }
}

impl<'a> Into<Item<'a>> for Content<'a> {
    fn into(self) -> Item<'a> {
        Item::Content(self)
    }
}

impl<'a> TypeName for Content<'a> {
    fn name() -> &'static str {
        "content"
    }
}