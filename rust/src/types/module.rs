use derive_more::Display;

use crate::{
    Item,
    types::{array::Array, string::String},
};

/// For more information visit the typst documentation: [module](https://typst.app/docs/reference/foundations/module/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Display)]
#[display("<module {}>", name)]
pub struct Module<'a> {
    #[serde(borrow)]
    pub name: String<'a>,
    pub member: Array<'a>,
}

crate::impl_all!(Item<'a>::Module, Module<'a>{'a}, "module");
