use std::fmt::Display;

use crate::{Item, TypstTypeLike};

/// For more information visit the typst documentation: [auto](https://typst.app/docs/reference/foundations/auto/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default, Hash)]
pub struct Auto;

crate::impl_all!(Item<'a>::Auto, Auto {}, "auto");

impl Display for Auto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as TypstTypeLike>::static_type_name())
    }
}
