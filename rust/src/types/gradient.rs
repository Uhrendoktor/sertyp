use crate::{
    Item,
    types::{
        angle::Angle,
        array::Array,
        color::Color,
        function::Function,
        generic::{AutoOr, Or},
        ratio::Ratio,
    },
};

/// For more information visit the typst documentation: [gradient](https://typst.app/docs/reference/visualize/gradient/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Gradient<'a> {
    /// For more information visit the typst documentation: [gradient.linear](https://typst.app/docs/reference/visualize/gradient/#definitions-linear)
    #[serde(rename = "gradient.linear")]
    Linear {
        #[serde(borrow)]
        stops: Or<Array<'a>, Color<'a>>,
        #[serde(borrow)]
        space: Function<'a>,
        relative: AutoOr<GradientRelative>,
        angle: Angle,
    },
    /// For more information visit the typst documentation: [gradient.radial](https://typst.app/docs/reference/visualize/gradient/#definitions-radial)
    #[serde(rename = "gradient.radial")]
    Radial {
        #[serde(borrow)]
        stops: Or<Array<'a>, Color<'a>>,
        #[serde(borrow)]
        space: Function<'a>,
        relative: AutoOr<GradientRelative>,
        #[serde(borrow)]
        center: Array<'a>,
        radius: Ratio,
        #[serde(rename = "focal-center")]
        focal_center: AutoOr<Array<'a>>,
        #[serde(rename = "focal-radius")]
        focal_radius: Ratio,
    },
    /// For more information visit the typst documentation: [gradient.conic](https://typst.app/docs/reference/visualize/gradient/#definitions-conic)
    #[serde(rename = "gradient.conic")]
    Conic {
        #[serde(borrow)]
        stops: Or<Array<'a>, Color<'a>>,
        #[serde(borrow)]
        space: Function<'a>,
        relative: AutoOr<GradientRelative>,
        #[serde(borrow)]
        center: Array<'a>,
        angle: Angle,
    },
}

crate::impl_all!(Item<'a>::Gradient, std::boxed::Box<Gradient<'a>> {'a}, "gradient");

crate::auto_impl_str! {
    /// For more information visit the typst documentation: [gradient.relative](https://typst.app/docs/reference/visualize/gradient/#definitions-relative)
    pub enum GradientRelative {
        Self_ = "self",
        Parent = "parent",
    }
}

impl<'a> Default for Gradient<'a> {
    fn default() -> Self {
        Gradient::Linear {
            stops: Or::default(),
            space: Function::default(),
            relative: AutoOr::default(),
            angle: Angle::default(),
        }
    }
}
