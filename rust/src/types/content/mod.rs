mod symbol;
mod sequence;
mod h;
mod v;
pub mod math;
use crate::types::content::{symbol::Symbol_};
pub use crate::types::content::{h::H, v::V, sequence::Sequence};
pub use crate::{Symbol, types::{dictionary::Dictionary, function::Function}};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(tag="func", content="fields", rename_all="lowercase")]
pub enum TypedContent<'a> {
    #[serde(borrow, rename="symbol")]
    Symbol(Symbol_<'a>),
    #[serde(borrow)]
    Sequence(Sequence<'a>),
    Space,
    H(H<'a>),
    V(V<'a>),
    #[serde(borrow, rename="math.accent")]
    MathAccent(math::Accent<'a>),
    #[serde(borrow, rename="math.attach")]
    MathAttach(math::Attach<'a>),
    #[serde(borrow, rename="math.binom")]
    MathBinom(math::Binom<'a>),
    #[serde(borrow, rename="math.cancel")]
    MathCancel(math::Cancel<'a>),
    #[serde(borrow, rename="math.cases")]
    MathCases(math::Cases<'a>),
    #[serde(borrow, rename="math.class")]
    MathClass(math::Class<'a>),
    #[serde(borrow, rename="math.equation")]
    MathEquation(math::Equation<'a>),
    #[serde(borrow, rename="math.frac")]
    MathFrac(math::Frac<'a>),
    #[serde(borrow, rename="math.lr")]
    MathLR(math::LR<'a>),
    #[serde(borrow, rename="math.mat")]
    MathMat(math::Mat<'a>),
    #[serde(borrow, rename="math.root")]
    MathRoot(math::Root<'a>),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct RawContent<'a> {
    #[serde(borrow)]
    pub func: Function<'a>,
    pub fields: Dictionary<'a>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Content_<'a> {
    #[serde(borrow)]
    Typed(TypedContent<'a>),
    Unknown(RawContent<'a>)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(from="Content_", into="Content_")]
#[serde(untagged)]
pub enum Content<'a> {
    #[serde(borrow)]
    Symbol(Symbol<'a>),
    Sequence(Sequence<'a>),
    Space,
    H(H<'a>),
    V(V<'a>),
    MathAccent(math::Accent<'a>),
    MathAttach(math::Attach<'a>),
    MathBinom(math::Binom<'a>),
    MathCancel(math::Cancel<'a>),
    MathCases(math::Cases<'a>),
    MathClass(math::Class<'a>),
    MathEquation(math::Equation<'a>),
    MathFrac(math::Frac<'a>),
    MathLR(math::LR<'a>),
    MathMat(math::Mat<'a>),
    MathRoot(math::Root<'a>),
    Other(RawContent<'a>),
}

crate::impl_all!(Content<'a>, "content");

impl<'a> From<Content_<'a>> for Content<'a> {
    fn from(value: Content_<'a>) -> Self {
        match value {
            Content_::Typed(typed) => match typed {
                TypedContent::Symbol(s) => Content::Symbol(s.into()),
                TypedContent::Sequence(s) => Content::Sequence(s),
                TypedContent::Space => Content::Space,
                TypedContent::H(h) => Content::H(h),
                TypedContent::V(v) => Content::V(v),
                TypedContent::MathAccent(r) => Content::MathAccent(r),
                TypedContent::MathAttach(r) => Content::MathAttach(r),
                TypedContent::MathBinom(r) => Content::MathBinom(r),
                TypedContent::MathCancel(r) => Content::MathCancel(r),
                TypedContent::MathCases(c) => Content::MathCases(c),
                TypedContent::MathClass(c) => Content::MathClass(c),
                TypedContent::MathEquation(r) => Content::MathEquation(r),
                TypedContent::MathFrac(r) => Content::MathFrac(r),
                TypedContent::MathLR(r) => Content::MathLR(r),
                TypedContent::MathMat(r) => Content::MathMat(r),

                TypedContent::MathRoot(r) => Content::MathRoot(r),
            },
            Content_::Unknown(raw) => Content::Other(raw),
        }
    }
}

impl<'a> Into<Content_<'a>> for Content<'a> {
    fn into(self) -> Content_<'a> {
        match self {
            Content::Symbol(s) => Content_::Typed(TypedContent::Symbol(s.into())),
            Content::Sequence(s) => Content_::Typed(TypedContent::Sequence(s)),
            Content::Space => Content_::Typed(TypedContent::Space),
            Content::H(h) => Content_::Typed(TypedContent::H(h)),
            Content::V(v) => Content_::Typed(TypedContent::V(v)),
            Content::MathAccent(r) => Content_::Typed(TypedContent::MathAccent(r)),
            Content::MathAttach(r) => Content_::Typed(TypedContent::MathAttach(r)),
            Content::MathBinom(r) => Content_::Typed(TypedContent::MathBinom(r)),
            Content::MathCancel(r) => Content_::Typed(TypedContent::MathCancel(r)),
            Content::MathCases(c) => Content_::Typed(TypedContent::MathCases(c)),
            Content::MathClass(c) => Content_::Typed(TypedContent::MathClass(c)),
            Content::MathEquation(r) => Content_::Typed(TypedContent::MathEquation(r)),
            Content::MathFrac(r) => Content_::Typed(TypedContent::MathFrac(r)),
            Content::MathLR(r) => Content_::Typed(TypedContent::MathLR(r)),
            Content::MathMat(r) => Content_::Typed(TypedContent::MathMat(r)),

            Content::MathRoot(r) => Content_::Typed(TypedContent::MathRoot(r)),
            Content::Other(raw) => Content_::Unknown(raw),
        }
    }
}

#[macro_export]
macro_rules! impl_try_from_content {
    ($variant:ident$(<$lt:lifetime>)*$(.$prefix:ident)*) => {
        paste::paste!{impl<'a> TryFrom<crate::Content<'a>> for $variant$(<$lt>)* {
            type Error = std::string::String;

            fn try_from(value: crate::Content<'a>) -> Result<Self, Self::Error> {
                match value {
                    crate::Content::[<$($prefix)* $variant>](v) => Ok(v),
                    _ => Err(format!("Tried to cast Content to {}, found {:?}", stringify!([<$($prefix)* $variant>]), value)),
                }
            }
        }}
    };
}

#[macro_export]
macro_rules! impl_into_content {
    ($variant:ident$(<$lt:lifetime>)*$(.$prefix:ident)*) => {
        paste::paste!{impl<'a> Into<crate::Content<'a>> for $variant$(<$lt>)* {
            fn into(self) -> crate::Content<'a> {
                crate::Content::[<$($prefix)* $variant>](self)
            }
        }}
    };
}

#[macro_export]
macro_rules! impl_content_name {
    ($variant:ident$(<$lt:lifetime>)*, $name:expr) => {
        impl$(<$lt>)* crate::TypstTypeLike for $variant$(<$lt>)* {
            fn static_type_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_all_content {
    ($variant:ident$(<$lt:lifetime>)*$(.$prefix:ident)*, $name:expr) => {
        crate::impl_try_from_content!($variant$(<$lt>)*$(.$prefix)*);
        crate::impl_into_content!($variant$(<$lt>)*$(.$prefix)*);
        crate::impl_content_name!($variant$(<$lt>)*, $name);
    };
}
