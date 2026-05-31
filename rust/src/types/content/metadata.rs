use crate::{Content, Item, Label, TypedItem};

/// For more information visit the typst documentation: [metadata](https://typst.app/docs/reference/introspection/metadata/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Metadata<'a> {
    #[serde(borrow)]
    pub value: Box<Item<'a>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub label: Option<TypedItem<Label<'a>>>,
}

crate::impl_all!(Content<'a>::Metadata, Metadata<'a>{'a}, "metadata");
