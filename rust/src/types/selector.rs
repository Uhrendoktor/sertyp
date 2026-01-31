use crate::Item;

/// Selectors are not yet supported due to serialization complexities.
///
/// For more information visit the typst documentation: [selector](https://typst.app/docs/reference/foundations/selector/)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct Selector;

crate::impl_all!(Item<'a>::Selector, Selector {}, "selector");
