use derive_more::{Deref, DerefMut};

use crate::types::Item;

/// Used within typst's internals to apply styles to `Content`. The stylistic aspects cannot yet be parsed as they are not exposed. This struct simply contains the wrapped [Item].
///
/// For more information visit the typst documentation: [styles](https://typst.app/docs/reference/math/styles/)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
pub struct Styles<'a>(#[serde(borrow)] pub Box<Item<'a>>);

crate::impl_all!(Item<'a>::Styles, Styles<'a>{'a}, "styles");
