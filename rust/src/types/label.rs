use std::fmt::Display;

use crate::types::string::String;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct Label<'a>(
    #[serde(borrow)]
    pub String<'a>
);

crate::impl_all!(Label<'a>, "label");

impl<'a> Display for Label<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}