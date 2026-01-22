use crate::{Fraction, Or, Relative, TypedItem, types::boolean::Boolean};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct V {
    pub amount: Or<Relative, Fraction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak: Option<TypedItem<Boolean>>
}

crate::impl_all_content!(V, "v");