use derive_more::{Deref, DerefMut, From, Into};

use crate::Item;

/// For more information visit the typst documentation: [bool](https://typst.app/docs/reference/foundations/bool/)
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Hash,
    Deref,
    DerefMut,
    From,
    Into,
)]
pub struct Boolean(pub bool);

crate::impl_all!(Item<'a>::Boolean, Boolean {}, "boolean");
