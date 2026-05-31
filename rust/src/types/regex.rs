use derive_more::{Deref, DerefMut, Display, From, Into};

use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [regex](https://typst.app/docs/reference/foundations/regex/)
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Default,
    Hash,
    Deref,
    DerefMut,
    From,
    Into,
    Display,
)]
#[display("/{}/", **self)]
pub struct Regex<'a>(#[serde(borrow)] pub String<'a>);

crate::impl_all!(Item<'a>::Regex, Regex<'a>{'a}, "regex");

impl<'a> From<&'a str> for Regex<'a> {
    fn from(value: &'a str) -> Self {
        Regex(String::from(value))
    }
}
