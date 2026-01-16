use std::fmt::Display;

use crate::TypstType;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Auto;

crate::impl_all!(Auto, "auto");

impl Display for Auto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as TypstType>::static_type_name())
    }
}