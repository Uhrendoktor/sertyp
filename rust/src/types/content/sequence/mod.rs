use crate::{Content, Item, types::generic::TypedArray};
use std::ops::{Deref, DerefMut};

pub mod locating;
pub use locating::LocatingSequence;
pub(crate) use locating::{GroupType, PreToken};

pub mod chumsky;
pub mod error;

/// Used within typst's internals to represent a space seperated sequence of different content items within a single content block. This is basically an array of `Content`.
/// # Example of Typst Behavior
/// ```typst
/// #let content = [a sentence with some math: $a+b=c$]
/// // is parsed as `sequence(([a sentence with some math] [:] space math.equation(...))`
/// ```
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Sequence<'a> {
    #[serde(borrow)]
    pub children: TypedArray<Content<'a>>,
}

impl<'a> Sequence<'a> {
    pub fn new() -> Self {
        Sequence {
            children: TypedArray::default(),
        }
    }
}

crate::impl_into!(Content<'a>::Sequence, Sequence<'a>);
crate::impl_typst_type!(Sequence<'a>{'a}, "sequence");

impl<'a> From<Content<'a>> for Sequence<'a> {
    fn from(content: Content<'a>) -> Self {
        match content {
            Content::Sequence(seq) => seq,
            other => Sequence {
                children: TypedArray::from(vec![other]),
            },
        }
    }
}

impl<'a> From<Vec<Content<'a>>> for Sequence<'a> {
    fn from(vec: Vec<Content<'a>>) -> Self {
        Sequence {
            children: TypedArray::from(vec),
        }
    }
}

impl<'a> Deref for Sequence<'a> {
    type Target = TypedArray<Content<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<'a> DerefMut for Sequence<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

/// Utility wrapper that enable auto serialization and deserialization for any type T that implements [TryFrom]<[Sequence]> and [Into]<[Sequence]>.
#[derive(Debug, Clone)]
pub struct TypedSequence<T>(pub T);

impl<T> Deref for TypedSequence<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TypedSequence<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> TypedSequence<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'a, T: TryFrom<Sequence<'a>>> TryFrom<Sequence<'a>> for TypedSequence<T> {
    type Error = T::Error;

    fn try_from(value: Sequence<'a>) -> std::result::Result<Self, Self::Error> {
        Ok(TypedSequence(value.try_into()?))
    }
}

impl<'a, T: TryFrom<Sequence<'a>>> TryFrom<Content<'a>> for TypedSequence<T> {
    type Error = T::Error;

    fn try_from(content: Content<'a>) -> std::result::Result<Self, Self::Error> {
        let seq: Sequence<'a> = content.into();
        seq.try_into()
    }
}

impl<'a, T: TryFrom<Sequence<'a>, Error = <Content<'a> as TryFrom<Item<'a>>>::Error>>
    TryFrom<Item<'a>> for TypedSequence<T>
{
    type Error = T::Error;

    fn try_from(item: Item<'a>) -> std::result::Result<Self, Self::Error> {
        let content: Content = item.try_into()?;
        content.try_into()
    }
}

impl<'a, T: Into<Sequence<'a>>> From<TypedSequence<T>> for Sequence<'a> {
    fn from(typed_seq: TypedSequence<T>) -> Self {
        typed_seq.0.into()
    }
}

impl<'a, T: Into<Sequence<'a>>> From<TypedSequence<T>> for Content<'a> {
    fn from(typed_seq: TypedSequence<T>) -> Self {
        Sequence::from(typed_seq).into()
    }
}

impl<'a, T: Into<Sequence<'a>>> From<TypedSequence<T>> for Item<'a> {
    fn from(typed_seq: TypedSequence<T>) -> Self {
        Content::from(typed_seq).into()
    }
}
