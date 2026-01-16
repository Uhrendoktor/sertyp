use crate::{Angle, AutoOr, Boolean, Box, Color, Content, Dictionary, Function, Gradient, Length, Or, Relative, Stroke, Tiling};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Cancel<'a> {
    #[serde(borrow)]
    pub body: Box<Content<'a>>,
    pub length: Relative,
    pub inverted: Boolean,
    pub cross: Boolean,
    #[serde(borrow)]
    pub angle: AutoOr<Or<Angle, Function<'a>>>,
    #[serde(borrow)]
    pub stroke: CancelStroke<'a>
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