use crate::{Boolean, Content, Or, Relative, String, TypedItem};

/// For more information visit the typst documentation: [math.accent](https://typst.app/docs/reference/math/accent/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Accent<'a> {
    #[serde(borrow)]
    pub base: TypedItem<Box<Content<'a>>>,
    pub accent: Or<String<'a>, Box<Content<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<TypedItem<Relative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dotless: Option<TypedItem<Boolean>>,
}

crate::impl_all!(Content<'a>::MathAccent, Accent<'a>{'a}, "math.accent");
