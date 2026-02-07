use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [decimal](https://typst.app/docs/reference/foundations/decimal/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq, Default, Hash)]
pub struct Decimal<'a>(#[serde(borrow)] pub String<'a>);

crate::impl_all!(Item<'a>::Decimal, Decimal<'a>{'a}, "decimal");

impl<'a> From<Decimal<'a>> for f64 {
    fn from(val: Decimal<'a>) -> Self {
        val.0.parse().unwrap()
    }
}
