use crate::{Angle, AutoOr, Boolean, Box, Color, Content, Dictionary, Function, Gradient, Length, Or, Relative, Stroke, Tiling, TypedItem};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Cancel<'a> {
    #[serde(borrow)]
    pub body: Box<TypedItem<Content<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<TypedItem<Relative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<TypedItem<Boolean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross: Option<TypedItem<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub angle: Option<AutoOr<Or<Angle, Function<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<CancelStroke<'a>>
}

crate::impl_all_content!(Cancel<'a>.Math, "math.cancel");

crate::auto_impl!{
    #[derive(Clone, Debug)]
    pub enum CancelStroke<'a> {
        Length(Length),
        Color(Color<'a>),
        Gradient(Gradient<'a>),
        Stroke(Stroke<'a>),
        Tiling(Tiling),
        Dictionary(Dictionary<'a>),
    }
}