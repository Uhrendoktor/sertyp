use crate::types::integer::Integer;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Default)]
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

crate::impl_all!(Datetime, "datetime");