use crate::{Content, Relative, TypedItem};

/// For more information visit the typst documentation: [math.stretch](https://typst.app/docs/reference/math/stretch/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Stretch<'a> {
    #[serde(borrow)]
    pub body: TypedItem<Box<Content<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<TypedItem<Relative>>,
}

crate::impl_all!(Content<'a>::MathStretch, Stretch<'a>{'a}, "math.stretch");
