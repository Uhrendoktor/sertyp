#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Boolean(pub bool);

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

crate::impl_all!(Boolean, "boolean");

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.0
    }
}