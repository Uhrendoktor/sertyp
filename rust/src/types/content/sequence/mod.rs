use crate::{Content, Item, types::generic::TypedArray};
use std::ops::{Deref, DerefMut};

pub mod locating;
use derive_more::{AsMut, AsRef, Deref, DerefMut, IntoIterator};
pub use locating::LocatingSequence;
pub(crate) use locating::{GroupType, PreToken};

pub mod chumsky;
pub mod error;

pub use error::TypstError;

/// Used within Typst's internals to represent a space separated sequence of different content items within a single content block. This is basically an array of `Content`.
/// # Example of Typst Behavior
/// ```typst
/// #let content = [a sentence with some math: $a+b=c$]
/// // is parsed as `sequence(([a sentence with some math] [:] space math.equation(...))`
/// ```
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Debug,
    Default,
    Deref,
    DerefMut,
    AsRef,
    AsMut,
    IntoIterator,
)]
#[deref(forward)]
#[deref_mut(forward)]
#[as_ref(forward)]
#[as_mut(forward)]
#[into_iterator(owned, ref, ref_mut)]
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

    pub fn flatten(self) -> Self {
        let mut children = vec![];
        for i in 0..self.children.len() {
            if let Content::Sequence(seq) = self.children[i].clone() {
                children.extend(seq.flatten().children);
            } else {
                children.push(self.children[i].clone());
            }
        }
        children.into()
    }
}

crate::impl_into!(Content<'a>::Sequence, Sequence<'a>);
crate::impl_typst_type!(Sequence<'a>{'a}, "sequence");

impl<'a> From<Sequence<'a>> for Item<'a> {
    fn from(seq: Sequence<'a>) -> Self {
        Content::Sequence(seq).into()
    }
}

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

#[macro_export]
macro_rules! sequence {
    ($($x:expr),*$(,)?) => {
        sertyp::Sequence::from(vec![$($x.into()),*])
    };
}

impl<'a> From<Vec<Content<'a>>> for Content<'a> {
    fn from(vec: Vec<Content<'a>>) -> Self {
        Sequence::from(vec).into()
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
