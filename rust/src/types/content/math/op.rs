use crate::{Boolean, Box, Content, TypedItem};

/// For more information visit the typst documentation: [math.op](https://typst.app/docs/reference/math/op/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Op<'a> {
    #[serde(borrow)]
    pub text: Box<Content<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<TypedItem<Boolean>>,
}

crate::impl_all!(Content<'a>::MathOp, Op<'a>{'a}, "math.op");
