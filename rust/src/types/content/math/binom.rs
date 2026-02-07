use crate::{Box, Content, TypedItem, types::generic::TypedArray};

/// For more information visit the typst documentation: [math.binom](https://typst.app/docs/reference/math/binom/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Binom<'a> {
    #[serde(borrow)]
    pub upper: TypedItem<Box<Content<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub lower: Option<TypedArray<Content<'a>>>,
}

crate::impl_all!(Content<'a>::MathBinom, Binom<'a>{'a}, "math.binom");
