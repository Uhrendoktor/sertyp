use crate::types::{Item, array::Array, dictionary::Dictionary, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Arguments<'a> {
    #[serde(borrow)]
    pub pos: Array<'a>,
    pub named: Dictionary<'a>
}

impl<'a> TryFrom<Item<'a>> for Arguments<'a> {
    type Error = std::string::String;
    
    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Arguments(a) => Ok(a),
            _ => Err(format!("Invalid type for Arguments: {:?}", value)),
        }
    }
}

impl<'a> From<Arguments<'a>> for Item<'a> {
    fn from(value: Arguments<'a>) -> Self {
        Item::Arguments(value)
    }
}

impl<'a> TypeName for Arguments<'a> {
    fn name() -> &'static str {
        "arguments"
    }
}

