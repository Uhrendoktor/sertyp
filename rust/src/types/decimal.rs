use crate::types::{Item, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Decimal<'a> (
    #[serde(borrow)]
    pub &'a str
);

impl<'a> Into<f64> for Decimal<'a> {
    fn into(self) -> f64 {
        self.0.parse().unwrap()
    }
}

impl<'a> TryFrom<Item<'a>> for Decimal<'a> {
    type Error = &'static str;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Decimal(d) => Ok(d),
            _ => Err("Invalid type for Decimal"),
        }
    }
}

impl<'a> Into<Item<'a>> for Decimal<'a> {
    fn into(self) -> Item<'a> {
        Item::Decimal(self)
    }
}

impl<'a> TypeName for Decimal<'a> {
    fn name() -> &'static str {
        "decimal"
    }
}