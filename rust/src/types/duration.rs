use std::ops::{Deref, DerefMut};

use crate::types::{Item, float::Float, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Duration(Float);

impl Deref for Duration {
    type Target = Float;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl<'a> TryFrom<Item<'a>> for Duration {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Duration(d) => Ok(d),
            _ => Err(format!("Invalid type for Duration: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Duration {
    fn into(self) -> Item<'a> {
        Item::Duration(self)
    }
}

impl<'a> TypeName for Duration {
    fn name() -> &'static str {
        "duration"
    }
}