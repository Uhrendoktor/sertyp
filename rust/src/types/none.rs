use derive_more::Display;

use crate::Item;

/// For more information visit the typst documentation: [none](https://typst.app/docs/reference/foundations/none/)
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default, Hash, Display,
)]
#[display("none")]
pub struct None;

crate::impl_all!(Item<'a>::None, None {}, "none");
