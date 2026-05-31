use crate::{
    Angle, AutoOr, Boolean, Color, Content, Dictionary, Function, Length, Or, Relative, Stroke,
    Tiling, TypedItem, types::Gradient,
};

/// For more information visit the typst documentation: [math.cancel](https://typst.app/docs/reference/math/cancel/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Cancel<'a> {
    #[serde(borrow)]
    pub body: TypedItem<Box<Content<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<TypedItem<Relative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<TypedItem<Boolean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross: Option<TypedItem<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub angle: Option<AutoOr<Or<Angle, Function<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<CancelStroke<'a>>,
}

crate::impl_all!(Content<'a>::MathCancel, Cancel<'a>{'a}, "math.cancel");

crate::auto_impl! {
    #[derive(Clone, Debug)]
    pub enum CancelStroke<'a> {
        try_from{},
        Length(Length=>Length),
        Color(Color=>Color<'a>),
        Gradient(Gradient=>Box<Gradient<'a>>),
        Stroke(Stroke=>Stroke<'a>),
        Tiling(Tiling=>Tiling),
        Dictionary(Dictionary=>Dictionary<'a>),
    }
}
