use crate::types::string::String;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct Decimal<'a> (
    #[serde(borrow)]
    pub String<'a>
);

crate::impl_all!(Decimal<'a>, "decimal");

impl<'a> Into<f64> for Decimal<'a> {
    fn into(self) -> f64 {
        self.0.parse().unwrap()
    }
}