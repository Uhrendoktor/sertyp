use std::fmt::Display;

use crate::types::{array::Array, string::String};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Module<'a> {
    #[serde(borrow)]
    pub name: String<'a>,
    pub member: Array<'a>,
}

crate::impl_all!(Module<'a>, "module");

impl<'a> Display for Module<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<module {}>", self.name)
    }
}