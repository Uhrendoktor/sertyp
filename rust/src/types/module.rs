use std::fmt::Display;

use crate::{
    Item,
    types::{array::Array, string::String},
};

/// For more information visit the typst documentation: [module](https://typst.app/docs/reference/foundations/module/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Module<'a> {
    #[serde(borrow)]
    pub name: String<'a>,
    pub member: Array<'a>,
}

crate::impl_all!(Item<'a>::Module, Module<'a>{'a}, "module");

impl<'a> Display for Module<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<module {}>", self.name)
    }
}
