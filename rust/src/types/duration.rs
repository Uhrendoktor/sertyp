use std::ops::{Deref, DerefMut};

use crate::types::float::Float;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Duration(pub Float);

impl Deref for Duration {
    type Target = Float;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

crate::impl_all!(Duration, "duration");

impl DerefMut for Duration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<std::time::Duration> for Duration {
    fn into(self) -> std::time::Duration {
        let seconds: f64 = self.0.into();
        let nanos = (seconds.fract() * 1_000_000_000.0) as u32;
        std::time::Duration::new(seconds.trunc() as u64, nanos)
    }
}