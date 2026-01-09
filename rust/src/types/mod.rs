mod string;
mod integer;

mod generic;

mod alignment;
mod angle;
mod arguments;
mod array;
mod color;
mod content;
mod datetime;
mod decimal;
mod dictionary;
mod direction;
mod duration;
mod float;
mod fraction;
mod function;
mod gradient;
mod label;
mod length;
mod module;
mod ratio;
mod regex;
mod relative;
mod selector;
mod stroke;
mod styles;
mod symbol;
mod tiling;
mod r#type;
mod version;

mod panic;

use serde::{Deserialize, Serialize};

pub use crate::types::r#type::TypeName;
pub use crate::types::{panic::Panic, alignment::Alignment, angle::Angle, arguments::Arguments, array::Array, color::Color, content::Content, datetime::Datetime, decimal::Decimal, dictionary::Dictionary, direction::Direction, duration::Duration, float::Float, fraction::Fraction, function::Function, gradient::Gradient, integer::Integer, label::Label, length::Length, module::Module, ratio::Ratio, regex::Regex, relative::Relative, string::String, stroke::Stroke, styles::Styles, symbol::Symbol, tiling::Tiling, r#type::Type, version::Version};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Item_<'a> {
    Array(Array<'a>),
    Boolean(bool),
    Integer(Integer),
    #[serde(borrow)]
    ByteArray(TypeByteArray_<'a>),
    Other(ItemTagged_<'a>),
}

#[derive(Clone, Debug)]
pub enum TypeByteArray_<'a> {
    String(String<'a>),
    Bytes(&'a [u8]),
}

impl<'a> Deserialize<'a> for TypeByteArray_<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct TypeByteArrayVisitor;

        impl<'de> serde::de::Visitor<'de> for TypeByteArrayVisitor {
            type Value = TypeByteArray_<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a byte array or string")
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TypeByteArray_::Bytes(v))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TypeByteArray_::String(String(v)))
            }
        }

        deserializer.deserialize_any(TypeByteArrayVisitor)
    }
}

impl<'a> Serialize for TypeByteArray_<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TypeByteArray_::String(s) => serializer.serialize_str(s),
            TypeByteArray_::Bytes(b) => serializer.serialize_bytes(b),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum ItemTagged_<'a> {
    Alignment(Alignment),
    Angle(Angle),
    #[serde(borrow)]
    Arguments(Arguments<'a>),
    Auto,
    Color(Color<'a>),
    Content(Content<'a>),
    Datetime(Datetime),
    Decimal(Decimal<'a>),
    Dictionary(Dictionary<'a>),
    Direction(Direction),
    Duration(Duration),
    Float(Float),
    Fraction(Fraction),
    Function(Function<'a>),
    Gradient(Gradient<'a>),
    Label(Label<'a>),
    Length(Length),
    Module(Module<'a>),
    None,
    Ratio(Ratio),
    Regex(Regex<'a>),
    Relative(Relative<'a>),
    Stroke(Stroke<'a>),
    Styles(Styles<'a>),
    Symbol(Symbol<'a>),
    Tiling(Tiling<'a>),
    Type(Type<'a>),
    Version(Version),

    Panic(Panic<'a>),
}

impl<'a> From<Item<'a>> for Item_<'a> {
    fn from(value: Item<'a>) -> Self {
        match value {
            Item::Array(a) => Item_::Array(a),
            Item::Boolean(b) => Item_::Boolean(b),
            Item::Bytes(b) => Item_::ByteArray(TypeByteArray_::Bytes(b)),
            Item::Integer(i) => Item_::Integer(i),
            Item::String(s) => Item_::ByteArray(TypeByteArray_::String(s)),

            Item::Alignment(a) => Item_::Other(ItemTagged_::Alignment(a)),
            Item::Angle(a) => Item_::Other(ItemTagged_::Angle(a)),
            Item::Arguments(a) => Item_::Other(ItemTagged_::Arguments(a)),
            Item::Auto => Item_::Other(ItemTagged_::Auto),
            Item::Color(c) => Item_::Other(ItemTagged_::Color(c)),
            Item::Content(content) => Item_::Other(ItemTagged_::Content(content)),
            Item::Datetime(d) => Item_::Other(ItemTagged_::Datetime(d)),
            Item::Decimal(dec) => Item_::Other(ItemTagged_::Decimal(dec)),
            Item::Dictionary(d) => Item_::Other(ItemTagged_::Dictionary(d)),
            Item::Direction(dir) => Item_::Other(ItemTagged_::Direction(dir)),
            Item::Duration(dur) => Item_::Other(ItemTagged_::Duration(dur)),
            Item::Float(f) => Item_::Other(ItemTagged_::Float(f)),
            Item::Fraction(frac) => Item_::Other(ItemTagged_::Fraction(frac)),
            Item::Function(func) => Item_::Other(ItemTagged_::Function(func)),
            Item::Gradient(grad) => Item_::Other(ItemTagged_::Gradient(grad)),
            Item::Label(l) => Item_::Other(ItemTagged_::Label(l)),
            Item::Length(l) => Item_::Other(ItemTagged_::Length(l)),
            Item::Module(m) => Item_::Other(ItemTagged_::Module(m)),
            Item::None => Item_::Other(ItemTagged_::None),
            Item::Ratio(ratio) => Item_::Other(ItemTagged_::Ratio(ratio)),
            Item::Regex(r) => Item_::Other(ItemTagged_::Regex(r)),
            Item::Relative(rel) => Item_::Other(ItemTagged_::Relative(rel)),
            Item::Stroke(s) => Item_::Other(ItemTagged_::Stroke(s)),
            Item::Styles(s) => Item_::Other(ItemTagged_::Styles(s)),
            Item::Symbol(s) => Item_::Other(ItemTagged_::Symbol(s)),
            Item::Tiling(t) => Item_::Other(ItemTagged_::Tiling(t)),
            Item::Type(t) => Item_::Other(ItemTagged_::Type(t)),
            Item::Version(v) => Item_::Other(ItemTagged_::Version(v)),
            
            Item::Panic(p) => Item_::Other(ItemTagged_::Panic(p)),
        }
    }
}

impl<'a> From<Item_<'a>> for Item<'a> {
    fn from(value: Item_<'a>) -> Self{
        match value {
            Item_::Array(a) => Item::Array(a),
            Item_::Boolean(b) => Item::Boolean(b),
            Item_::Integer(i) => Item::Integer(i),
            Item_::ByteArray(ba) => match ba {
                TypeByteArray_::String(s) => Item::String(s),
                TypeByteArray_::Bytes(b) => Item::Bytes(b),
            },
            Item_::Other(o) => match o {
                ItemTagged_::Alignment(a) => Item::Alignment(a),
                ItemTagged_::Angle(a) => Item::Angle(a),
                ItemTagged_::Arguments(a) => Item::Arguments(a),
                ItemTagged_::Auto => Item::Auto,
                ItemTagged_::Color(c) => Item::Color(c),
                ItemTagged_::Content(c) => Item::Content(c),
                ItemTagged_::Datetime(d) => Item::Datetime(d),
                ItemTagged_::Decimal(dec) => Item::Decimal(dec),
                ItemTagged_::Dictionary(d) => Item::Dictionary(d),
                ItemTagged_::Direction(dir) => Item::Direction(dir),
                ItemTagged_::Duration(dur) => Item::Duration(dur),
                ItemTagged_::Float(f) => Item::Float(f),
                ItemTagged_::Fraction(frac) => Item::Fraction(frac),
                ItemTagged_::Function(func) => Item::Function(func),
                ItemTagged_::Gradient(grad) => Item::Gradient(grad),
                ItemTagged_::Label(l) => Item::Label(l),
                ItemTagged_::Length(l) => Item::Length(l),
                ItemTagged_::Module(m) => Item::Module(m),
                ItemTagged_::None => Item::None,
                ItemTagged_::Ratio(ratio) => Item::Ratio(ratio),
                ItemTagged_::Regex(r) => Item::Regex(r),
                ItemTagged_::Relative(rel) => Item::Relative(rel),
                ItemTagged_::Stroke(s) => Item::Stroke(s),
                ItemTagged_::Styles(s) => Item::Styles(s),
                ItemTagged_::Symbol(s) => Item::Symbol(s),
                ItemTagged_::Tiling(t) => Item::Tiling(t),
                ItemTagged_::Type(t) => Item::Type(t),
                ItemTagged_::Version(v) => Item::Version(v),
                
                ItemTagged_::Panic(p) => Item::Panic(p)
            },
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "Item_", into = "Item_")]
pub enum Item<'a> {
    Array(Array<'a>),
    Boolean(bool),
    Bytes(&'a [u8]),
    Integer(Integer),
    String(String<'a>),

    Alignment(Alignment),
    Angle(Angle),
    Arguments(Arguments<'a>),
    Auto,
    Color(Color<'a>),
    Content(Content<'a>),
    Datetime(Datetime),
    Decimal(Decimal<'a>),
    Dictionary(Dictionary<'a>),
    Direction(Direction),
    Duration(Duration),
    Float(Float),
    Fraction(Fraction),
    Function(Function<'a>),
    Gradient(Gradient<'a>),
    Label(Label<'a>),
    Length(Length),
    Module(Module<'a>),
    None,
    Ratio(Ratio),
    Regex(Regex<'a>),
    Relative(Relative<'a>),
    Stroke(Stroke<'a>),
    Styles(Styles<'a>),
    Symbol(Symbol<'a>),
    Tiling(Tiling<'a>),
    Type(Type<'a>),
    Version(Version),

    Panic(Panic<'a>),
}

impl TypeName for Item<'_> {
    fn name() -> &'static str {
        "item"
    }

    fn type_name(&self) -> &'static str {
        match self {
            Item::Array(_) => Array::name(),
            Item::Boolean(_) => "boolean",
            Item::Bytes(_) => "bytes",
            Item::Integer(_) => Integer::name(),
            Item::String(_) => String::name(),

            Item::Alignment(_) => Alignment::name(),
            Item::Angle(_) => Angle::name(),
            Item::Arguments(_) => Arguments::name(),
            Item::Auto => "auto",
            Item::Color(_) => Color::name(),
            Item::Content(_) => Content::name(),
            Item::Datetime(_) => Datetime::name(),
            Item::Decimal(_) => Decimal::name(),
            Item::Dictionary(_) => Dictionary::name(),
            Item::Direction(_) => Direction::name(),
            Item::Duration(_) => Duration::name(),
            Item::Float(_) => Float::name(),
            Item::Fraction(_) => Fraction::name(),
            Item::Function(_) => <Function as TypeName>::name(),
            Item::Gradient(_) => Gradient::name(),
            Item::Label(_) => Label::name(),
            Item::Length(_) => Length::name(),
            Item::Module(_) => Module::name(),
            Item::None => "none",
            Item::Ratio(_) => Ratio::name(),
            Item::Regex(_) => Regex::name(),
            Item::Relative(_) => Relative::name(),
            Item::Stroke(_) => Stroke::name(),
            Item::Styles(_) => Styles::name(),
            Item::Symbol(_) => Symbol::name(),
            Item::Tiling(_) => Tiling::name(),
            Item::Type(_) => Type::name(),
            Item::Version(_) => Version::name(),

            Item::Panic(_) => Panic::name(),
        }
    }
}