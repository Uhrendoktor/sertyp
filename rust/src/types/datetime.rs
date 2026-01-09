use crate::types::{integer::Integer, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Datetime {
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<Integer>,
}

impl<'a> TryFrom<crate::types::Item<'a>> for Datetime {
    type Error = std::string::String;

    fn try_from(value: crate::types::Item<'a>) -> Result<Self, Self::Error> {
        match value {
            crate::types::Item::Datetime(d) => Ok(d),
            _ => Err(format!("Invalid type for Datetime: {:?}", value)),
        }
    }
}

impl<'a> Into<crate::types::Item<'a>> for Datetime {
    fn into(self) -> crate::types::Item<'a> {
        crate::types::Item::Datetime(self)
    }
}

impl<'a> TypeName for Datetime {
    fn name() -> &'static str {
        "datetime"
    }
}