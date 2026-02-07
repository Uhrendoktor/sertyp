use std::fmt::Display;

use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [label](https://typst.app/docs/reference/foundations/label/)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default, Hash)]
pub struct Label<'a>(#[serde(borrow)] pub String<'a>);

crate::impl_all!(Item<'a>::Label, Label<'a>{'a}, "label");

impl<'a> Display for Label<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}
