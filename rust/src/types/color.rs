use crate::{Item, types::array::Array};

/// For more information visit the typst documentation: [color](https://typst.app/docs/reference/visualize/color/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Color<'a> {
    #[serde(borrow)]
    pub components: Array<'a>,
    pub space: ColorSpace,
}

crate::impl_all!(Item<'a>::Color, Color<'a>{'a}, "color");

crate::auto_impl_func! {
    /// For more information visit the typst documentation: [color space](https://typst.app/docs/reference/visualize/color/#definitions-space)
    #[derive(Default)]
    pub enum ColorSpace {
        #[default]
        Rgb = "color.rgb",
        Cmyk = "color.cmyk",
        Luma = "color.luma",
        OkLab = "color.oklab",
        OkLch = "color.oklch",
        LinearRgb = "color.linear-rgb",
        Hsl = "color.hsl",
        Hsv = "color.hsv"
    }
}
