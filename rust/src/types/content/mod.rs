#[cfg(feature = "content")]
mod h;
#[cfg(feature = "content")]
pub mod math;
#[cfg(feature = "content")]
mod metadata;
#[cfg(feature = "content")]
mod raw;
#[cfg(feature = "content")]
mod sequence;
#[cfg(feature = "content")]
mod space;
#[cfg(feature = "content")]
mod stack;
#[cfg(feature = "content")]
mod strong;
#[cfg(feature = "content")]
mod symbol;
#[cfg(feature = "content")]
mod text;
#[cfg(feature = "content")]
mod v;

#[cfg(feature = "content")]
use crate::types::content::{symbol::Symbol_};
#[cfg(feature = "content")]
pub use crate::types::content::{space::Space, raw::Raw, strong::Strong,stack::Stack, metadata::Metadata, h::H, v::V, sequence::Sequence, text::Text};
#[cfg(feature = "content")]
pub use crate::Symbol;


pub use crate::types::{dictionary::Dictionary, function::Function};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct RawContent<'a> {
    #[serde(borrow)]
    pub func: Function<'a>,
    pub fields: Dictionary<'a>,
}

#[cfg(not(feature = "content"))]
pub type Content<'a> = RawContent<'a>;

#[cfg(feature = "content")]
crate::define_enum! {
    #[serde(tag="func", content="fields", rename_all="lowercase")]
    pub enum Content<'a> {
        untagged {
            Unknown(RawContent<'a>),
        },
        remap {
            #[serde(borrow, rename="symbol")]
            Symbol(Symbol_<'a>) => Symbol(Symbol<'a>),
            Space => Space(Space),
        },
        H(H),
        #[serde(borrow)]
        Metadata(Metadata<'a>),
        #[serde(borrow)]
        Raw(Raw<'a>),
        #[serde(borrow)]
        Sequence(Sequence<'a>),
        #[serde(borrow)]
        Stack(Stack<'a>),
        #[serde(borrow)]
        Strong(Strong<'a>),
        #[serde(borrow)]
        Text(Text<'a>),
        V(V),

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
        #[serde(rename="math.primes")]
        MathPrimes(math::Primes),
        #[serde(borrow, rename="math.root")]
        MathRoot(math::Root<'a>),
        #[serde(borrow, rename="math.stretch")]
        MathStretch(math::Stretch<'a>),
        #[serde(borrow, rename="math.styled")]
        MathStyled(math::Styled<'a>),
        #[serde(borrow, rename="math.op")]
        MathOp(math::Op<'a>),
        #[serde(borrow, rename="math.vec")]
        MathVec(math::Vec<'a>),
    }
}

crate::impl_all!(Content<'a>, "content");

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
