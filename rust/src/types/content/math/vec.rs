use crate::{Alignment, Content, Relative, TypedItem, types::{content::math::generic::Delim, generic::TypedArray}};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Vec<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub delim: Option<Delim<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<TypedItem<Alignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<TypedItem<Relative>>,
    #[serde(borrow)]
    pub children: TypedArray<Content<'a>>,
}

crate::impl_all_content!(Vec<'a>.Math, "math.vec");
