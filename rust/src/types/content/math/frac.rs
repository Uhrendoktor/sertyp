use crate::{Content, RBox, TypedItem};

/// For more information visit the typst documentation: [math.frac](https://typst.app/docs/reference/math/frac/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Frac<'a> {
    #[serde(borrow)]
    pub num: TypedItem<RBox<Content<'a>>>,
    #[serde(borrow)]
    pub denom: TypedItem<RBox<Content<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<TypedItem<FracStyle>>,
}

crate::impl_all!(Content<'a>::MathFrac, Frac<'a>{'a}, "math.frac");

crate::auto_impl_str!(
    pub enum FracStyle {
        Vertical = "vertical",
        Skewed = "skewed",
        Horizontal = "horizontal",
    }
);
