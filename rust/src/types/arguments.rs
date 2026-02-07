use crate::{
    Item,
    types::{array::Array, dictionary::Dictionary},
};

/// For more information visit the typst documentation: [arguments](https://typst.app/docs/reference/foundations/arguments/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Arguments<'a> {
    #[serde(borrow)]
    pub pos: Array<'a>,
    pub named: Dictionary<'a>,
}

crate::impl_all!(Item<'a>::Arguments, Arguments<'a>{'a}, "arguments");
