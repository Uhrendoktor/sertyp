use crate::{Fraction, Or, Relative, impl_all_content, types::boolean::Boolean};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct H<'a> {
    #[serde(borrow)]
    pub amount: Or<Relative<'a>, Fraction>,
    pub weak: Boolean
}

impl_all_content!(H<'a>, "h");