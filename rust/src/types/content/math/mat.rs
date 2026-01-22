use crate::{Alignment, Content, Dictionary, Integer, Or, Relative, TypedItem, types::{content::math::generic::Delim, generic::TypedArray}};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Mat<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub delim: Option<Delim<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<TypedItem<Alignment>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub augment: Option<Or<Integer, Dictionary<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<TypedItem<Relative>>,
    #[serde(rename = "row-gap", skip_serializing_if = "Option::is_none")]
    pub row_gap: Option<TypedItem<Relative>>,
    #[serde(rename = "column-gap", skip_serializing_if = "Option::is_none")]
    pub column_gap: Option<TypedItem<Relative>>,
    #[serde(borrow)]
    pub rows: TypedArray<TypedArray<Content<'a>>>,
}

crate::impl_all_content!(Mat<'a>.Math, "math.mat");
