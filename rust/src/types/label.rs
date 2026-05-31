use derive_more::{Deref, DerefMut, Display};

use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [label](https://typst.app/docs/reference/foundations/label/)
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Hash,
    Display,
    Deref,
    DerefMut,
)]
#[display("<{}>", self.0)]
pub struct Label<'a>(#[serde(borrow)] pub String<'a>);

crate::impl_all!(Item<'a>::Label, Label<'a>{'a}, "label");
