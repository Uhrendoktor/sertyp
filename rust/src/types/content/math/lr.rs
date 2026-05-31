use crate::{Content, Relative, TypedItem};

/// For more information visit the typst documentation: [math.lr](https://typst.app/docs/reference/math/lr/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct LR<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<TypedItem<Relative>>,
    #[serde(borrow)]
    pub body: TypedItem<Box<Content<'a>>>,
}

crate::impl_all!(Content<'a>::MathLR, LR<'a>{'a}, "math.lr");
