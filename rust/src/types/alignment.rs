use crate::types::{Item, r#type::TypeName};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Start,
    End,
    Left,
    Center,
    Right,
    Top,
    Horizon,
    Bottom,
}

impl<'a> TryFrom<Item<'a>> for Alignment {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Alignment(a) => Ok(a),
            _ => Err(format!("Invalid type for Alignment: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Alignment {
    fn into(self) -> Item<'a> {
        Item::Alignment(self)
    }
}

impl<'a> TypeName for Alignment {
    fn name() -> &'static str {
        "alignment"
    }
}