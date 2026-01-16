use crate::{Fraction, Or, Relative, types::boolean::Boolean};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct V {
    pub amount: Or<Relative, Fraction>,
    pub weak: Boolean
}

crate::impl_all_content!(V, "v");