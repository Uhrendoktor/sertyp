use crate::{Length, types::{Item, array::Pair, r#type::TypeName}};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tiling<'a>{
    size: Pair<'a, Length>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    spacing: Option<Pair<'a, Length>>
}

impl<'a> TryFrom<Item<'a>> for Tiling<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Tiling(t) => Ok(t),
            _ => Err(format!("Unknown Tiling value: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Tiling<'a> {
    fn into(self) -> Item<'a> {
        Item::Tiling(self)
    }
}

impl<'a> TypeName for Tiling<'a> {
    fn name() -> &'static str {
        "tiling"
    }
}