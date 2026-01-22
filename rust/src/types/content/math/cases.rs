use crate::{Boolean, Content, Relative, TypedItem, types::{content::math::generic::Delim, generic::TypedArray}};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Cases<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub delim: Option<Delim<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<TypedItem<Boolean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<TypedItem<Relative>>,
    #[serde(borrow)]
    pub children: TypedArray<Content<'a>>,
}

crate::impl_all_content!(Cases<'a>.Math, "math.cases");