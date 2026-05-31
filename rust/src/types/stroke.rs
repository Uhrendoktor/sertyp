use crate::{
    AutoOr, Float, Item, Length,
    types::{array::Array, dictionary::Dictionary, generic::FillColor},
};

/// For more information visit the typst documentation: [stroke](https://typst.app/docs/reference/visualize/stroke/)
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

crate::auto_impl_str! {
    /// For more information visit the typst documentation: [stroke.cap](https://typst.app/docs/reference/visualize/stroke/#constructor-cap)
    pub enum StrokeCap {
        Butt = "butt",
        Round = "round",
        Square = "square",
    }
}

crate::auto_impl_str! {
    /// For more information visit the typst documentation: [stroke.join](https://typst.app/docs/reference/visualize/stroke/#constructor-join)
    pub enum StrokeJoin {
        Miter = "miter",
        Round = "round",
        Bevel = "bevel",
    }
}

crate::auto_impl_str! {
    /// For more information visit the typst documentation: [stroke.dash](https://typst.app/docs/reference/visualize/stroke/#constructor-dash)
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
    /// For more information visit the typst documentation: [stroke.dash](https://typst.app/docs/reference/visualize/stroke/#constructor-dash)
    #[derive(Debug, Clone)]
    pub enum StrokeDash<'a> {
        try_from {
            Variant(StrokeDashVariant),
        },
        Array(Array=>Array<'a>),
        Dictionary(Dictionary=>Dictionary<'a>),
    }
);

crate::impl_all!(Item<'a>::Stroke, Stroke<'a>{'a}, "stroke");
