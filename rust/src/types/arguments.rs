use crate::types::{array::Array, dictionary::Dictionary};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Arguments<'a> {
    #[serde(borrow)]
    pub pos: Array<'a>,
    pub named: Dictionary<'a>
}

crate::impl_all!(Arguments<'a>, "arguments");
