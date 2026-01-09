use crate::types::{Item, array::Array, color::Color, dictionary::Dictionary, gradient::Gradient, string::String, tiling::Tiling, r#type::TypeName};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Stroke<'a> {
    #[serde(borrow)]
    paint: Box<Item<'a>>,
    thickness: Box<Item<'a>>,
    cap: Box<Item<'a>>,
    join: Box<Item<'a>>,
    dash: Box<Item<'a>>,
    #[serde(rename = "miter-limit")]
    miter_limit: Box<Item<'a>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum StrokeCap {
    Auto,
    Butt,
    Round,
    Square,
}

impl<'a> TryFrom<Item<'a>> for StrokeCap {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Auto => Ok(StrokeCap::Auto),
            Item::String(s) => match &*s {
                "butt" => Ok(StrokeCap::Butt),
                "round" => Ok(StrokeCap::Round),
                "square" => Ok(StrokeCap::Square),
                _ => Err(format!("Unknown StrokeCap value: {}", &*s)),
            },
            _ => Err(format!("Expected String type for StrokeCap, got {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for StrokeCap {
    fn into(self) -> Item<'a> {
        match self {
            StrokeCap::Auto => Item::Auto,
            StrokeCap::Butt => String::from("butt").into(),
            StrokeCap::Round => String::from("round").into(),
            StrokeCap::Square => String::from("square").into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StrokeJoin {
    Auto,
    Miter,
    Round,
    Bevel,
}

impl<'a> TryFrom<Item<'a>> for StrokeJoin {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Auto => Ok(StrokeJoin::Auto),
            Item::String(s) => match &*s {
                "miter" => Ok(StrokeJoin::Miter),
                "round" => Ok(StrokeJoin::Round),
                "bevel" => Ok(StrokeJoin::Bevel),
                _ => Err(format!("Unknown StrokeJoin value: {}", &*s)),
            },
            _ => Err(format!("Expected String type for StrokeJoin, got {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for StrokeJoin {
    fn into(self) -> Item<'a> {
        match self {
            StrokeJoin::Auto => Item::Auto,
            StrokeJoin::Miter => String::from("miter").into(),
            StrokeJoin::Round => String::from("round").into(),
            StrokeJoin::Bevel => String::from("bevel").into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(try_from = "Item", into = "Item")]
pub enum StrokePaint<'a> {
    Auto,
    #[serde(borrow)]
    Color(Color<'a>),
    Gradient(Gradient<'a>),
    Tiling(Tiling<'a>)
}

impl<'a> TryFrom<Item<'a>> for StrokePaint<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Auto => Ok(StrokePaint::Auto),
            Item::Color(c) => Ok(StrokePaint::Color(c)),
            Item::Gradient(g) => Ok(StrokePaint::Gradient(g)),
            Item::Tiling(t) => Ok(StrokePaint::Tiling(t)),
            _ => Err(format!("Unknown StrokePaint value: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for StrokePaint<'a> {
    fn into(self) -> Item<'a> {
        match self {
            StrokePaint::Auto => Item::Auto,
            StrokePaint::Color(c) => Item::Color(c),
            StrokePaint::Gradient(g) => Item::Gradient(g),
            StrokePaint::Tiling(t) => Item::Tiling(t),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(try_from = "Item", into = "Item")]
pub enum StrokeDash<'a>{
    Auto,
    None,
    Solid,
    Dotted,
    Dashed,
    DenselyDashed,
    LooselyDashed,
    DashDotted,
    DenselyDashDotted,
    LooselyDashDotted,
    #[serde(borrow)]
    Array(Array<'a>),
    Dictionary(Dictionary<'a>),
}

impl<'a> TryFrom<Item<'a>> for StrokeDash<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Auto => Ok(StrokeDash::Auto),
            Item::String(s) => match &*s {
                "none" => Ok(StrokeDash::None),
                "solid" => Ok(StrokeDash::Solid),
                "dotted" => Ok(StrokeDash::Dotted),
                "dashed" => Ok(StrokeDash::Dashed),
                "densely-dashed" => Ok(StrokeDash::DenselyDashed),
                "loosely-dashed" => Ok(StrokeDash::LooselyDashed),
                "dash-dotted" => Ok(StrokeDash::DashDotted),
                "densely-dash-dotted" => Ok(StrokeDash::DenselyDashDotted),
                "loosely-dash-dotted" => Ok(StrokeDash::LooselyDashDotted),
                _ => Err(format!("Unknown StrokeDash value: {}", &*s)),
            },
            Item::Array(a) => Ok(StrokeDash::Array(a)),
            Item::Dictionary(d) => Ok(StrokeDash::Dictionary(d)),
            _ => Err(format!("Expected String, Array or Dictionary type for StrokeDash, got {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for StrokeDash<'a> {
    fn into(self) -> Item<'a> {
        match self {
            StrokeDash::Auto => Item::Auto,
            StrokeDash::None => String::from("none").into(),
            StrokeDash::Solid => String::from("solid").into(),
            StrokeDash::Dotted => String::from("dotted").into(),
            StrokeDash::Dashed => String::from("dashed").into(),
            StrokeDash::DenselyDashed => String::from("densely-dashed").into(),
            StrokeDash::LooselyDashed => String::from("loosely-dashed").into(),
            StrokeDash::DashDotted => String::from("dash-dotted").into(),
            StrokeDash::DenselyDashDotted => String::from("densely-dash-dotted").into(),
            StrokeDash::LooselyDashDotted => String::from("loosely-dash-dotted").into(),
            StrokeDash::Array(a) => Item::Array(a),
            StrokeDash::Dictionary(d) => Item::Dictionary(d),
        }
    }
}


impl<'a> TryFrom<Item<'a>> for Stroke<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Stroke(l) => Ok(l),
            _ => Err(format!("Invalid type for Stroke: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Stroke<'a> {
    fn into(self) -> Item<'a> {
        Item::Stroke(self)
    }
}

impl<'a> TypeName for Stroke<'a> {
    fn name() -> &'static str {
        "stroke"
    }
}