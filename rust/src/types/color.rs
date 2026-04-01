use crate::{Item, Ratio, TypedArray, types::array::Array};

/// For more information visit the typst documentation: [color](https://typst.app/docs/reference/visualize/color/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Color<'a> {
    #[serde(borrow)]
    pub components: Array<'a>,
    pub space: ColorSpace,
}

crate::impl_all!(Item<'a>::Color, Color<'a>{'a}, "color");

crate::auto_impl_func! {
    /// For more information visit the typst documentation: [color space](https://typst.app/docs/reference/visualize/color/#definitions-space)
    #[derive(Default, Hash)]
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

impl<'a> Color<'a> {
    pub fn rgba(r: Ratio, g: Ratio, b: Ratio, a: Ratio) -> Self {
        let a: TypedArray<_> = vec![r, g, b, a].into();
        Color {
            components: a.into(),
            space: ColorSpace::Rgb,
        }
    }
    pub fn rgba_hex(mut hex: &str) -> Result<Self, String> {
        if hex.starts_with("#") {
            hex = &hex[1..];
        }
        if hex.len() != 6 && hex.len() != 8 {
            return Err(format!("Invalid hex color: {hex:#?}"));
        }
        let (r, g, b) = (
            u8::from_str_radix(&hex[0..2], 16).map_err(|e| format!("{e:#?}"))?,
            u8::from_str_radix(&hex[2..4], 16).map_err(|e| format!("{e:#?}"))?,
            u8::from_str_radix(&hex[4..6], 16).map_err(|e| format!("{e:#?}"))?,
        );
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).map_err(|e| format!("{e:#?}"))?
        } else {
            255
        };
        Ok(Color::rgba(
            Ratio::percent255(r),
            Ratio::percent255(g),
            Ratio::percent255(b),
            Ratio::percent255(a),
        ))
    }
}
