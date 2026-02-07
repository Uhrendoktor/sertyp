use std::fmt::Display;

use crate::{Item, TypstTypeLike};

/// For more information visit the typst documentation: [none](https://typst.app/docs/reference/foundations/none/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default, Hash)]
pub struct None;

crate::impl_all!(Item<'a>::None, None {}, "none");

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as TypstTypeLike>::static_type_name())
    }
}
