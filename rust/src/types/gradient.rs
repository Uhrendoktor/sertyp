use crate::types::{angle::Angle, array::Array, color::Color, function::Function, generic::{AutoOr, Or}, ratio::Ratio};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Gradient<'a> {
    #[serde(rename = "gradient.linear")]
    Linear {
        #[serde(borrow)]
        stops: Or<Array<'a>, Color<'a>>,
        #[serde(borrow)]
        space: Function<'a>,
        relative: AutoOr<GradientRelative>,
        angle: Angle,
    },
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
        focal_radius: Ratio
    },
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
    }
}

crate::impl_all!(Gradient<'a>, "gradient");

crate::auto_impl_str!{
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