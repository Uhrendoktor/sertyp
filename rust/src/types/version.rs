use std::fmt::Display;

use crate::Integer;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Version {
    pub major: Integer,
    pub minor: Integer,
    pub patch: Integer
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

crate::impl_all!(Version, "version");