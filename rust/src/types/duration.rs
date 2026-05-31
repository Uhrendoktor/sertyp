use derive_more::{Deref, DerefMut, From, Into};

use crate::{Item, types::float::Float};

/// For more information visit the typst documentation: [duration](https://typst.app/docs/reference/foundations/duration/)
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Debug,
    PartialEq,
    Default,
    Deref,
    DerefMut,
    Into,
    From,
)]
pub struct Duration(pub Float);

crate::impl_all!(Item<'a>::Duration, Duration {}, "duration");

impl From<Duration> for std::time::Duration {
    fn from(val: Duration) -> Self {
        let seconds: f64 = val.0.into();
        let nanos = (seconds.fract() * 1_000_000_000.0) as u32;
        std::time::Duration::new(seconds.trunc() as u64, nanos)
    }
}
