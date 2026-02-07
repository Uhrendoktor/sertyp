use crate::{Content, Fraction, Or, Relative, TypedItem, types::boolean::Boolean};

/// For more information visit the typst documentation: [h](https://typst.app/docs/reference/layout/h/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct H {
    pub amount: Or<Relative, Fraction>, // parses as Item::Length but expects Item::Relative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak: Option<TypedItem<Boolean>>,
}

crate::impl_all!(Content<'a>::H, H {}, "h");
