use crate::Item;

/// For more information visit the typst documentation: [bool](https://typst.app/docs/reference/foundations/bool/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Boolean(pub bool);

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

crate::impl_all!(Item<'a>::Boolean, Boolean {}, "boolean");

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.0
    }
}
