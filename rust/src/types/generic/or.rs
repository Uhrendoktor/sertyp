use serde::{Deserialize, Serialize};

use crate::{Auto, Content, Item, None, TypstTypeLike};
#[cfg(feature = "content")]
use crate::{ParsedSequence, Sequence, SequenceView, StructuredSequence};

/// Utility for specifying a type that can be one of two possible [Item] variants. In case more variants are needed, consider using [crate::auto_impl].
/// # Example
/// ```
/// use sertyp::{typst_func, Or, Integer, String};
///
/// //#[typst_func]
/// fn somefn<'a>(arg: Or<Integer, String<'a>>) -> String<'a> {
///     match arg {
///         Or::Left(v) => format!("Integer: {}", v),
///         Or::Right(v) => format!("String: {}", v),
///     }.into()
/// }
/// ```
#[derive(Clone, Debug, Hash)]
pub enum Or<T1, T2> {
    Left(T1),
    Right(T2),
}

/// Utility for types that are either [Auto] or a specific type. For more information see [Or].
pub type AutoOr<T> = Or<Auto, T>;
/// Utility for types that are either [None] or a specific type. For more information see [Or].
#[allow(dead_code)]
pub type NoneOr<T> = Or<None, T>;

impl<T1, T2> Default for Or<T1, T2>
where
    T1: Default,
{
    fn default() -> Self {
        Or::Left(T1::default())
    }
}

impl<T1, T2> Or<T1, T2> {
    pub fn is_left(&self) -> bool {
        matches!(self, Or::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, Or::Right(_))
    }

    pub fn into_left(self) -> Option<T1> {
        match self {
            Or::Left(v) => Some(v),
            _ => std::option::Option::None,
        }
    }

    pub fn into_right(self) -> Option<T2> {
        match self {
            Or::Right(v) => Some(v),
            _ => std::option::Option::None,
        }
    }

    pub fn left(&self) -> Option<&T1> {
        match self {
            Or::Left(v) => Some(v),
            _ => std::option::Option::None,
        }
    }

    pub fn right(&self) -> Option<&T2> {
        match self {
            Or::Right(v) => Some(v),
            _ => std::option::Option::None,
        }
    }

    pub fn left_or(self, default: T1) -> T1 {
        match self {
            Or::Left(v) => v,
            _ => default,
        }
    }

    pub fn right_or(self, default: T2) -> T2 {
        match self {
            Or::Right(v) => v,
            _ => default,
        }
    }
}

impl<'a, 'de: 'a, T1, T2> Deserialize<'de> for Or<T1, T2>
where
    Or<T1, T2>: TryFrom<Item<'a>>,
    <Or<T1, T2> as TryFrom<Item<'a>>>::Error: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item = Item::deserialize(deserializer)?;
        Or::try_from(item).map_err(serde::de::Error::custom)
    }
}

impl<'a, T1: Clone + Into<Item<'a>>, T2: Clone + Into<Item<'a>>> Serialize for Or<T1, T2>
where
    Or<T1, T2>: Into<Item<'a>>,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let item: Item = self.clone().into();
        item.serialize(serializer)
    }
}

macro_rules! impl_conversion {
    (
        $name:ident$(<$($g:tt),*>)?
    )=> {
        impl<
            $($($g),*,)?
            T1: Clone + TryFrom<$name$(<$($g),*>)?> + TypstTypeLike,
            T2: Clone + TryFrom<$name$(<$($g),*>)?> + TypstTypeLike,
        > TryFrom<$name$(<$($g),*>)?> for Or<T1, T2>
        {
            type Error = std::string::String;

            fn try_from(value: $name$(<$($g),*>)?) -> std::result::Result<Self, Self::Error> {
                let value_typ = <_ as TypstTypeLike>::type_name(&value);
                T1::try_from(value.clone())
                    .map(Or::Left)
                    .or_else(|_| T2::try_from(value).map(Or::Right))
                    .map_err(|_| {
                        format!(
                            "Type was expected to be {}, found {}",
                            <Self as TypstTypeLike>::static_type_name(),
                            value_typ
                        )
                    })
            }
        }

        impl<$($($g),*,)? T1: Into<$name$(<$($g),*>)?>, T2: Into<$name$(<$($g),*>)?>> From<Or<T1, T2>> for $name$(<$($g),*>)? {
            fn from(val: Or<T1, T2>) -> Self {
                match val {
                    Or::Left(t1) => t1.into(),
                    Or::Right(t2) => t2.into(),
                }
            }
        }
    }
}

impl_conversion! { Item<'a> }
impl_conversion! { Content<'a> }
#[cfg(feature = "content")]
impl_conversion! { Sequence<'a> }
#[cfg(feature = "content")]
impl_conversion! { StructuredSequence<'a> }

#[cfg(feature = "content")]
impl<T1, T2> SequenceView for Or<T1, T2>
where
    T1: SequenceView,
    T2: SequenceView,
{
    fn parse<'a>(
        seq: StructuredSequence<'a>,
    ) -> std::result::Result<ParsedSequence<'a, Or<T1, T2>>, std::string::String> {
        if let Ok(parsed) = T1::parse(seq.clone()) {
            return Ok(ParsedSequence {
                value: Or::Left(parsed.value),
                remaining: parsed.remaining,
            });
        }
        let parsed = T2::parse(seq)?;
        Ok(ParsedSequence {
            value: Or::Right(parsed.value),
            remaining: parsed.remaining,
        })
    }
}

impl<T1, T2> TypstTypeLike for Or<T1, T2>
where
    T1: TypstTypeLike,
    T2: TypstTypeLike,
{
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!(
            "{}|{}",
            T1::static_type_name(),
            T2::static_type_name()
        ))
    }
}
