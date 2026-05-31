use derive_more::Display;

use crate::Item;

/// For more information visit the typst documentation: [auto](https://typst.app/docs/reference/foundations/auto/)
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default, Hash, Display,
)]
#[display("auto")]
pub struct Auto;

crate::impl_all!(Item<'a>::Auto, Auto {}, "auto");
