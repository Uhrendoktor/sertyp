use crate::types::{Item, angle::Angle, array::Array, color::Color, function::Function, generic::{AutoOr, Or}, ratio::Ratio, string::String, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Gradient<'a> {
    #[serde(rename = "gradient.linear")]
    Linear {
        #[serde(borrow)]
        stops: Or<'a, Array<'a>, Color<'a>>,
        space: Function<'a>,
        relative: AutoOr<'a, GradientRelative>,
        angle: Angle,
    },
    #[serde(rename = "gradient.radial")]
    Radial {
        #[serde(borrow)]
        stops: Or<'a, Array<'a>, Color<'a>>,
        space: Function<'a>,
        relative: AutoOr<'a, GradientRelative>,
        center: Array<'a>,
        radius: Ratio,
        #[serde(rename = "focal-center")]
        focal_center: AutoOr<'a, Array<'a>>,
        #[serde(rename = "focal-radius")]
        focal_radius: Ratio
    },
    #[serde(rename = "gradient.conic")]
    Conic {
        #[serde(borrow)]
        stops: Or<'a, Array<'a>, Color<'a>>,
        space: Function<'a>,
        relative: AutoOr<'a, GradientRelative>,
        center: Array<'a>,
        angle: Angle,
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(try_from = "String", into = "String")]
pub enum GradientRelative {
    Self_,
    Parent,
}

impl<'a> TryFrom<String<'a>> for GradientRelative {
    type Error = std::string::String;

    fn try_from(value: String<'a>) -> Result<Self, Self::Error> {
        match &*value {
            "self" => Ok(GradientRelative::Self_),
            "parent" => Ok(GradientRelative::Parent),
            v => Err(format!("Invalid GradientRelative value: {}", v)),
        }
    }
}

impl<'a> TryFrom<Item<'a>> for GradientRelative {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::String(s) => GradientRelative::try_from(s),
            _ => Err("Expected String for GradientRelative".to_string()),
        }
    }
}

impl<'a> From<GradientRelative> for String<'a> {
    fn from(value: GradientRelative) -> Self {
        match value {
            GradientRelative::Self_ => String::from("self"),
            GradientRelative::Parent => String::from("parent"),
        }
    }
}

impl<'a> From<GradientRelative> for Item<'a> {
    fn from(value: GradientRelative) -> Self {
        let s: String<'a> = value.into();
        Item::String(s)
    }
}


#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(try_from = "Function", into = "Function")]
pub enum ColorSpace {
    RGB,
    CMYK,
    Luma,
    OkLab,
    OkLch,
    LinearRGB,
    HSL,
    HSV
}

impl<'a> TryFrom<Function<'a>> for ColorSpace {
    type Error = std::string::String;

    fn try_from(func: Function<'a>) -> Result<Self, Self::Error> {
        match func.full_name() {
            Some("color.rgb") => Ok(ColorSpace::RGB),
            Some("color.cmyk") => Ok(ColorSpace::CMYK),
            Some("color.luma") => Ok(ColorSpace::Luma),
            Some("color.oklab") => Ok(ColorSpace::OkLab),
            Some("color.oklch") => Ok(ColorSpace::OkLch),
            Some("color.linear-rgb") => Ok(ColorSpace::LinearRGB),
            Some("color.hsl") => Ok(ColorSpace::HSL),
            Some("color.hsv") => Ok(ColorSpace::HSV),
            _ => Err(format!("Unknown color space function: {:?}", func))
        }
    }
}

impl<'a> From<ColorSpace> for Function<'a> {
    fn from(space: ColorSpace) -> Self {
        match space {
            ColorSpace::RGB => Function::Named("color.rgb"),
            ColorSpace::CMYK => Function::Named("color.cmyk"),
            ColorSpace::Luma => Function::Named("color.luma"),
            ColorSpace::OkLab => Function::Named("color.oklab"),
            ColorSpace::OkLch => Function::Named("color.oklch"),
            ColorSpace::LinearRGB => Function::Named("color.linear-rgb"),
            ColorSpace::HSL => Function::Named("color.hsl"),
            ColorSpace::HSV => Function::Named("color.hsv"),
        }
    }
}

impl<'a> TryFrom<Item<'a>> for Gradient<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Gradient(g) => Ok(g),
            _ => Err(format!("Invalid type for Gradient: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Gradient<'a> {
    fn into(self) -> Item<'a> {
        Item::Gradient(self)
    }
}

impl<'a> TypeName for Gradient<'a> {
    fn name() -> &'static str {
        "gradient"
    }
}