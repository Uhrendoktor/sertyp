use crate::types::{Item, array::Array, function::Function, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Color<'a> {
    #[serde(borrow)]
    pub components: Array<'a>,
    pub space: ColorSpace
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

impl<'a> TryFrom<Item<'a>> for Color<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Color(c) => Ok(c),
            _ => Err(format!("Invalid type for Color: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Color<'a> {
    fn into(self) -> Item<'a> {
        Item::Color(self)
    }
}

impl<'a> TypeName for Color<'a> {
    fn name() -> &'static str {
        "color"
    }
}