use std::{ops::Deref};

use crate::types::string::String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Symbol<'a>(
    #[serde(borrow)]
    pub String<'a>
);

impl<'a> Deref for Symbol<'a> {
    type Target = String<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

crate::impl_all!(Symbol<'a>, "symbol");