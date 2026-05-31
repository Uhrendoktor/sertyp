use derive_more::Display;

use crate::{Integer, Item};

/// For more information visit the typst documentation: [version](https://typst.app/docs/reference/foundations/version/)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Hash, Display)]
#[display("{}.{}.{}", major, minor, patch)]
pub struct Version {
    pub major: Integer,
    pub minor: Integer,
    pub patch: Integer,
}

crate::impl_all!(Item<'a>::Version, Version {}, "version");
