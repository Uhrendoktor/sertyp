use crate::types::string::String;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Panic<'a> (
    #[serde(borrow)]
    pub String<'a>
);

crate::impl_all!(Panic<'a>, "panic");

impl<'a> From<String<'a>> for Panic<'a> {
    fn from(value: String<'a>) -> Self {
        Panic(value)
    }
}