use derive_more::{Display, IsVariant};

use crate::{Item, types::float::Float};

/// For more information visit the typst documentation: [ratio](https://typst.app/docs/reference/layout/ratio/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default, Display)]
#[display("{}{}", value, unit)]
pub struct Ratio {
    pub value: Float,
    pub unit: RatioUnit,
}

crate::impl_all!(Item<'a>::Ratio, Ratio {}, "ratio");

impl Ratio {
    /// Creates a [Ratio] for values in the range 0 = 0% to 1 = 100%.
    pub fn percent1(value: impl Into<Float>) -> Self {
        let f: f64 = value.into().into();
        Ratio {
            value: (f * 100.0).into(),
            unit: RatioUnit::Percent,
        }
    }

    /// Creates a [Ratio] for values in the range 0 = 0% to 1 = 100%.
    pub fn percent100(value: impl Into<Float>) -> Self {
        let f: f64 = value.into().into();
        Ratio {
            value: (f).into(),
            unit: RatioUnit::Percent,
        }
    }

    /// Creates a [Ratio] for values in the range 0 = 0% to 255 = 100%.
    pub fn percent255(value: impl Into<Float>) -> Self {
        let f: f64 = value.into().into();
        Ratio {
            value: (f / 255.0 * 100.0).into(),
            unit: RatioUnit::Percent,
        }
    }
}

/// Unit of a ratio value. Typst only supports percentages (`%`).
#[derive(
    Default,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    IsVariant,
    Display,
)]
pub enum RatioUnit {
    #[default]
    #[serde(rename = "%")]
    #[display("%")]
    Percent,
}
