use crate::{AutoOr, Float, Length, types::{array::Array, dictionary::Dictionary, generic::FillColor}};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Stroke<'a> {
    #[serde(borrow)]
    pub paint: AutoOr<FillColor<'a>>,
    pub thickness: AutoOr<Length>,
    pub cap: AutoOr<StrokeCap>,
    pub join: AutoOr<StrokeJoin>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub dash: Option<AutoOr<StrokeDash<'a>>>,
    #[serde(rename = "miter-limit")]
    pub miter_limit: AutoOr<Float>,
}

crate::auto_impl_str!{
    pub enum StrokeCap {
        Butt = "butt",
        Round = "round",
        Square = "square",
    }
}

crate::auto_impl_str!{
    pub enum StrokeJoin {
        Miter = "miter",
        Round = "round",
        Bevel = "bevel",
    }
}

crate::auto_impl_str!{
    pub enum StrokeDashVariant {
        Solid = "solid",
        Dotted = "dotted",
        Dashed = "dashed",
        DenselyDashed = "densely-dashed",
        LooselyDashed = "loosely-dashed",
        DashDotted = "dash-dotted",
        DenselyDashDotted = "densely-dash-dotted",
        LooselyDashDotted = "loosely-dash-dotted",
    }
}

crate::auto_impl!(
    #[derive(Debug, Clone)]
    pub enum StrokeDash<'a> {
        #[try_from]
        Variant(StrokeDashVariant),
        Array(Array<'a>),
        Dictionary(Dictionary<'a>),
    }
);


crate::impl_all!(Stroke<'a>, "stroke");