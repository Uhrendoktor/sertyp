use crate::{Fraction, Or, Relative, TypedItem, impl_all_content, types::boolean::Boolean};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct H {
    pub amount: Or<Relative, Fraction>, // parses as Item::Length but expects Item::Relative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak: Option<TypedItem<Boolean>>
}

impl_all_content!(H, "h");