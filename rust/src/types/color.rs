use crate::types::array::Array;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Color<'a> {
    #[serde(borrow)]
    pub components: Array<'a>,
    pub space: ColorSpace
}

crate::impl_all!(Color<'a>, "color");

crate::auto_impl_func!{
    pub enum ColorSpace {
        RGB = "color.rgb",
        CMYK = "color.cmyk",
        Luma = "color.luma",
        OkLab = "color.oklab",
        OkLch = "color.oklch",
        LinearRGB = "color.linear-rgb",
        HSL = "color.hsl",
        HSV = "color.hsv"
    }
}

impl Default for ColorSpace {
    fn default() -> Self {
        ColorSpace::RGB
    }
}