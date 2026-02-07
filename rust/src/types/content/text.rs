use crate::{Array, Content, Dictionary, String};

use crate::{
    AutoOr, Boolean, Direction, Integer, Item, Length, Or, Ratio, Relative, TypedItem,
    types::generic::{FillColor, StrokeColor},
};

/// For more information visit the typst documentation: [text](https://typst.app/docs/reference/text/text/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Text<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub font: Option<TextFont<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<TypedItem<Boolean>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<TextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Or<Integer, TextWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stretch: Option<TypedItem<Ratio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<TypedItem<Length>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillColor<'a>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<StrokeColor<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<TypedItem<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<TypedItem<Relative>>,
    #[serde(
        borrow,
        rename = "cjk-spacing-latin",
        skip_serializing_if = "Option::is_none"
    )]
    pub cjk_spacing_latin: Option<std::boxed::Box<Item<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TypedItem<Length>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhang: Option<TypedItem<Boolean>>,
    #[serde(rename = "top-edge", skip_serializing_if = "Option::is_none")]
    pub top_edge: Option<Or<Length, TextTopEdge>>,
    #[serde(rename = "bottom-edge", skip_serializing_if = "Option::is_none")]
    pub bottom_edge: Option<Or<Length, TextBottomEdge>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub lang: Option<TypedItem<String<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub region: Option<TypedItem<String<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub script: Option<AutoOr<String<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<AutoOr<Direction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyphenate: Option<AutoOr<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub costs: Option<TypedItem<Dictionary<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerning: Option<TypedItem<Boolean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternates: Option<TypedItem<Boolean>>,
    #[serde(
        borrow,
        rename = "stylistic-set",
        skip_serializing_if = "Option::is_none"
    )]
    pub sylistic_set: Option<Or<Integer, Array<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lignatures: Option<TypedItem<Boolean>>,
    #[serde(
        rename = "discretionary-ligatures",
        skip_serializing_if = "Option::is_none"
    )]
    pub discretionary_ligatures: Option<TypedItem<Boolean>>,
    #[serde(
        rename = "historical-ligatures",
        skip_serializing_if = "Option::is_none"
    )]
    pub historical_ligatures: Option<TypedItem<Boolean>>,
    #[serde(rename = "number-type", skip_serializing_if = "Option::is_none")]
    pub number_type: Option<AutoOr<TextNumberType>>,
    #[serde(rename = "number-width", skip_serializing_if = "Option::is_none")]
    pub number_width: Option<AutoOr<TextNumberWidth>>,
    #[serde(rename = "slashed-zero", skip_serializing_if = "Option::is_none")]
    pub slashed_zero: Option<TypedItem<Boolean>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fractions: Option<TypedItem<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub features: Option<Or<Array<'a>, Dictionary<'a>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub body: Option<TypedItem<Content<'a>>>,
    #[serde(borrow)]
    pub text: TypedItem<String<'a>>,
}

impl<'a> Text<'a> {
    pub fn from_string<S: Into<String<'a>>>(s: S) -> Self {
        Text {
            text: TypedItem(s.into()),
            ..Default::default()
        }
    }

    pub fn as_string(&self) -> &String<'a> {
        &self.text.0
    }
}

crate::auto_impl! {
    #[derive(Debug, Clone, Hash)]
    pub enum TextFont<'a> {
        try_from{},
        String(String=>String<'a>),
        Array(Array=>Array<'a>),
        Dictionary(Dictionary=>Dictionary<'a>),
    }
}
impl Default for TextFont<'static> {
    fn default() -> Self {
        TextFont::String(String::from("libertinus serif"))
    }
}

crate::auto_impl_str! {
    #[derive(Default)]
    pub enum TextStyle {
        #[default]
        Normal = "normal",
        Italic = "italic",
        Oblique = "oblique",
    }
}

crate::auto_impl_str! {
    pub enum TextWeight {
        Thin = "thin",
        ExtraLight = "extralight",
        Light = "light",
        Regular = "regular",
        Medium = "medium",
        SemiBold = "semibold",
        Bold = "bold",
        ExtraBold = "extrabold",
        Black = "black",
    }
}

crate::auto_impl_str! {
    pub enum TextTopEdge {
        Ascender = "ascender",
        CapHeight = "cap-height",
        XHeight = "x-height",
        Baseline = "baseline",
        Bounds = "bounds",
    }
}

crate::auto_impl_str! {
    pub enum TextBottomEdge {
        Baseline = "baseline",
        Descender = "descender",
        Bounds = "bounds",
    }
}

crate::auto_impl_str! {
    pub enum TextNumberType {
        Lining = "lining",
        OldStyle = "old-style",
    }
}

crate::auto_impl_str! {
    pub enum TextNumberWidth {
        Proportional = "proportional",
        Tabular = "tabular",
    }
}

crate::impl_all!(Content<'a>::Text, std::boxed::Box<Text<'a>>{'a}, "text");

impl<'a> From<Text<'a>> for Content<'a> {
    fn from(val: Text<'a>) -> Self {
        Content::Text(val.into())
    }
}

impl<'a> TryFrom<Content<'a>> for Text<'a> {
    type Error = <std::boxed::Box<Text<'a>> as TryFrom<Content<'a>>>::Error;

    fn try_from(value: Content<'a>) -> Result<Self, Self::Error> {
        let text: std::boxed::Box<Text<'a>> = value.try_into()?;
        Ok(text.into())
    }
}

impl<'a> From<std::boxed::Box<Text<'a>>> for Text<'a> {
    fn from(val: std::boxed::Box<Text<'a>>) -> Self {
        *val
    }
}
