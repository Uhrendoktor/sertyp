use crate::{Item, Label, impl_all_content};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Metadata<'a> {
    #[serde(borrow)]
    pub value: std::boxed::Box<Item<'a>>,
    #[serde(borrow)]
    pub label: Label<'a>
}

impl_all_content!(Metadata<'a>, "metadata");