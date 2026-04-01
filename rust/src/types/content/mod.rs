#[cfg(feature = "content")]
mod r#box;
#[cfg(feature = "content")]
mod h;
#[cfg(feature = "content")]
pub mod math;
#[cfg(feature = "content")]
mod metadata;
#[cfg(feature = "content")]
mod parbreak;
#[cfg(feature = "content")]
mod place;
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

use crate::Item;
#[cfg(feature = "content")]
pub use crate::types::content::{
    r#box::Box, h::H, metadata::Metadata, place::Place, raw::Raw, sequence::*, space::Space,
    stack::Stack, strong::Strong, text::Text, v::V,
};
#[cfg(feature = "content")]
use crate::{
    Panic, Symbol,
    types::content::{parbreak::Parbreak, symbol::Symbol_},
};

pub use crate::types::{dictionary::Dictionary, function::Function};

/// the raw presentation of any content. This is the default type if the `content` feature is not enabled.
/// Typst represents content as a `function` with `arguments`, which renders arbitrary content.
///
/// For more information visit the typst documentation: [content](https://typst.app/docs/reference/foundations/content/)
#[derive(Default, serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
pub struct RawContent<'a> {
    #[serde(borrow)]
    pub func: Function<'a>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Dictionary<'a>>,
}

#[cfg(not(feature = "content"))]
pub type Content<'a> = RawContent<'a>;
#[cfg(not(feature = "content"))]
pub type ItemContent<'a> = RawContent<'a>;

#[cfg(feature = "content")]
pub type ItemContent<'a> = std::boxed::Box<Content<'a>>;
#[cfg(feature = "content")]
crate::impl_into_typed!(Item, Content<'a>);

crate::impl_all!(typst_like Item<'a>::Content, ItemContent<'a>{'a}, "content");
#[cfg(feature = "content")]
crate::impl_typst_type!(Content<'a>{'a}, "content");

#[cfg(feature = "content")]
impl<'a> From<ItemContent<'a>> for Content<'a> {
    fn from(val: ItemContent<'a>) -> Self {
        *val
    }
}

#[cfg(feature = "content")]
impl<'a> TryFrom<Item<'a>> for Content<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        let content: std::boxed::Box<Content<'a>> = value.try_into()?;
        Ok(content.into())
    }
}

#[cfg(feature = "content")]
impl<'a> From<Content<'a>> for Item<'a> {
    fn from(val: Content<'a>) -> Self {
        let content: std::boxed::Box<Content<'a>> = std::boxed::Box::new(val);
        content.into()
    }
}

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
        Box(std::boxed::Box<Box<'a>>),
        H(H),
        #[serde(borrow)]
        Metadata(Metadata<'a>),
        Parbreak(Parbreak),
        #[serde(borrow)]
        Place(Place<'a>),
        #[serde(borrow)]
        Raw(Raw<'a>),
        #[serde(borrow)]
        Sequence(Sequence<'a>),
        #[serde(borrow)]
        Stack(Stack<'a>),
        #[serde(borrow)]
        Strong(Strong<'a>),
        #[serde(borrow)]
        Text(std::boxed::Box<Text<'a>>),
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
        MathMatrix(math::Matrix<'a>),
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
        MathVector(math::Vector<'a>),

        Panic(Panic<'a>),
    }
}

#[cfg(feature = "content")]
impl<'a> Default for Content<'a> {
    fn default() -> Self {
        std::boxed::Box::new(Text::default()).into()
    }
}

#[cfg(feature = "content")]
impl<'a, T: TryFrom<Content<'a>>> TryFrom<Item<'a>> for TypedContent<T>
where
    T::Error: std::fmt::Display,
{
    type Error = String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        let content: Content<'a> = value.try_into()?;
        let typed: T = content
            .try_into()
            .map_err(|e: <T as TryFrom<Content<'a>>>::Error| e.to_string())?;
        Ok(TypedContent::new(typed))
    }
}

#[cfg(feature = "content")]
impl<'a, T: Into<Content<'a>>> From<TypedContent<T>> for Item<'a> {
    fn from(val: TypedContent<T>) -> Self {
        let content: Content<'a> = val.into();
        content.into()
    }
}
