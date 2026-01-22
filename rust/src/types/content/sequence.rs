use crate::{Content, types::generic::TypedArray};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Sequence<'a> {
    #[serde(borrow)]
    pub children: TypedArray<Content<'a>>
}

crate::impl_all_content!(Sequence<'a>, "sequence");