use crate::Content;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Sequence<'a> {
    #[serde(borrow)]
    pub children: Vec<Content<'a>>
}

crate::impl_all_content!(Sequence<'a>, "sequence");