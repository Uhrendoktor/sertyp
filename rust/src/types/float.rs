use std::fmt::Display;


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum Float{
    // f32(f32),
    f64(f64),
}

crate::impl_all!(Float, "float");

impl Into<f64> for Float {
    fn into(self) -> f64 {
        match self {
            // Float::f32(f) => f as f64,
            Float::f64(f) => f,
        }
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float::f64(value)
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Float::f32(val) => val.fmt(f),
            Float::f64(val) => val.fmt(f),
        }
    }
}