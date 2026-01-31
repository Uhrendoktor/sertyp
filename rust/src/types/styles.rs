use std::ops::{Deref, DerefMut};

use crate::types::Item;

/// Used within typst's internals to apply styles to `Content`. The stylistic aspects cannot yet be parsed as they are not exposed. This struct simply contains the wrapped [Item].
///
/// For more information visit the typst documentation: [styles](https://typst.app/docs/reference/math/styles/)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Styles<'a>(#[serde(borrow)] pub Box<Item<'a>>);

impl<'a> Deref for Styles<'a> {
    type Target = Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Styles<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

crate::impl_all!(Item<'a>::Styles, Styles<'a>{'a}, "styles");
