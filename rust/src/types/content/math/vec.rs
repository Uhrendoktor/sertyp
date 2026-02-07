use std::ops::Index;

use crate::{
    Alignment, Content, Relative, TypedItem,
    types::{content::math::generic::Delim, generic::TypedArray},
};

/// For more information visit the typst documentation: [math.vec](https://typst.app/docs/reference/math/vec/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Vector<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub delim: Option<Delim<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<TypedItem<Alignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<TypedItem<Relative>>,
    #[serde(borrow)]
    pub children: TypedArray<Content<'a>>,
}

crate::impl_all!(Content<'a>::MathVector, Vector<'a>{'a}, "math.vec");

impl<'a> Index<usize> for Vector<'a> {
    type Output = Content<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index]
    }
}
