
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct Selector;

crate::impl_all!(Selector, "selector");