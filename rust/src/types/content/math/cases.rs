use crate::{
    Boolean, Content, Relative, TypedItem,
    types::{content::math::generic::Delim, generic::TypedArray},
};

/// For more information visit the typst documentation: [math.cases](https://typst.app/docs/reference/math/cases/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
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

crate::impl_all!(Content<'a>::MathCases, Cases<'a>{'a}, "math.cases");
