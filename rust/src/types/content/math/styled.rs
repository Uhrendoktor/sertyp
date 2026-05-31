use crate::{Content, TypedItem};

/// Used within typst's internals to apply styles to `Content`. The stylistic aspects cannot yet be parsed as they are not exposed. The `child` simply contains the wrapped content.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Styled<'a> {
    #[serde(borrow)]
    pub child: TypedItem<Box<Content<'a>>>,
}

crate::impl_all!(Content<'a>::MathStyled, Styled<'a>{'a}, "math.styled");
