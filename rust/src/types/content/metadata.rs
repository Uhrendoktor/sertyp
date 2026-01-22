use crate::{Item, Label, TypedItem, impl_all_content};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Metadata<'a> {
    #[serde(borrow)]
    pub value: std::boxed::Box<Item<'a>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub label: Option<TypedItem<Label<'a>>>,
}

impl_all_content!(Metadata<'a>, "metadata");