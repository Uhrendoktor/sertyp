use std::{fmt::Debug, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

use crate::{TypstTypeLike, types::Item};

#[derive(Clone, Debug)]
pub enum Or<
    T1,
    T2
> {
    Type1(T1),
    Type2(T2),
}

impl<'a, 'de: 'a, T1, T2> Deserialize<'de> for Or<T1, T2>
where
    Self: 'a,
    Or<T1, T2>: TryFrom<Item<'a>, Error=std::string::String>,
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

impl<
    'a,
    T1: Clone + TryFrom<Item<'a>, Error=std::string::String> + TypstTypeLike,
    T2: Clone + TryFrom<Item<'a>, Error=std::string::String> + TypstTypeLike,
> TryFrom<Item<'a>> for Or<T1, T2> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        let value_typ = <_ as TypstTypeLike>::type_name(&value);
        T1::try_from(value.clone()).map(Or::Type1)
            .or_else(|_| T2::try_from(value).map(Or::Type2))
            .map_err(|_| format!(
                "Type was expected to be {}, found {}",
                <Self as TypstTypeLike>::static_type_name(),
                value_typ
            ).into())
    }
}

impl<
    'a,
    T1: Into<Item<'a>>,
    T2: Into<Item<'a>>,
> Into<Item<'a>> for Or<T1, T2> {
    fn into(self) -> Item<'a> {
        match self {
            Or::Type1(t1) => t1.into(),
            Or::Type2(t2) => t2.into(),
        }
    }
}

impl<T1, T2> TypstTypeLike for Or<T1, T2>
where
    T1: TypstTypeLike,
    T2: TypstTypeLike,
{
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("{}|{}", T1::static_type_name(), T2::static_type_name()))
    }
}

#[derive(Clone, Debug)]
pub enum AutoOr<
    T
> {
    Auto,
    Type(T),
}

impl<'a, 'de: 'a, T> Deserialize<'de> for AutoOr<T>
where
    AutoOr<T>: TryFrom<Item<'a>, Error=std::string::String>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item = Item::deserialize(deserializer)?;
        AutoOr::try_from(item).map_err(serde::de::Error::custom)
    }
}

impl<'a, T: Clone + Into<Item<'a>>> Serialize for AutoOr<T>
where
    AutoOr<T>: Into<Item<'a>>,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let item: Item = self.clone().into();
        item.serialize(serializer)
    }
}

impl<
    'a,
    T: TryFrom<Item<'a>, Error=std::string::String> + TypstTypeLike,
> TryFrom<Item<'a>> for AutoOr<T> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        match T::try_from(value.clone()).map(AutoOr::Type) {
            Ok(v) => Ok(v),
            Err(_) => match value {
                Item::Auto => Ok(AutoOr::Auto),
                _ => Err(format!(
                    "Type was excepted to be {} found {}",
                    <Self as TypstTypeLike>::static_type_name(),
                    <_ as TypstTypeLike>::type_name(&value)
                )),
            },
        }
    }
}

impl<
    'a,
    T: Into<Item<'a>>,
> Into<Item<'a>> for AutoOr<T> {
    fn into(self) -> Item<'a> {
        match self {
            AutoOr::Type(t) => t.into(),
            AutoOr::Auto => Item::Auto,
        }
    }
}

impl<T> TypstTypeLike for AutoOr<T>
where
    T: TypstTypeLike,
{
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("auto|{}", T::static_type_name()))
    }
}

pub struct Result<'a, T>(pub std::result::Result<T, crate::types::String<'a>>);
impl<'a, T> From<std::result::Result<T, crate::types::String<'a>>> for Result<'a, T> {
    fn from(value: std::result::Result<T, crate::types::String<'a>>) -> Self {
        Result(value)
    }
}
impl<'a, T> Into<Item<'a>> for Result<'a, T>
where
    T: Into<Item<'a>>,
{
    fn into(self) -> Item<'a> {
        match self.0 {
            Ok(v) => v.into(),
            Err(e) => Item::Panic(e.into()),
        }
    }
}

impl<'a, T> Into<Item<'a>> for std::result::Result<T, crate::types::String<'a>>
where
    T: Into<Item<'a>>,
{
    fn into(self) -> Item<'a> {
        Result::<'_, T>::from(self).into()
    }
}

pub struct Box<T>(pub std::boxed::Box<T>);
impl<'a, 'de: 'a, T> Deserialize<'de> for Box<T>
where
    Self: 'a,
    Box<T>: TryFrom<Item<'a>, Error=std::string::String>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item = Item::deserialize(deserializer)?;
        Box::try_from(item).map_err(serde::de::Error::custom)
    }
}

impl<'a, T: Clone + Into<Item<'a>>> Serialize for Box<T>
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let item: Item = (&**self).clone().into();
        item.serialize(serializer)
    }
}

impl<T: Clone> Clone for Box<T> {
    fn clone(&self) -> Self {
        Box(std::boxed::Box::new((**self).clone()))
    }
}

impl<T: Debug> Debug for Box<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<std::boxed::Box<T>> for Box<T> {
    fn from(value: std::boxed::Box<T>) -> Self {
        Box(value)
    }
}

impl<'a, T: Clone + Into<Item<'a>>> Into<Item<'a>> for Box<T> {
    fn into(self) -> Item<'a> {
        let value: T = (&*self).clone();
        value.into()
    }
}

impl<
    'a,
    T: TryFrom<Item<'a>, Error=std::string::String>,
> TryFrom<Item<'a>> for Box<T> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        T::try_from(value.clone()).map(|t| Box(std::boxed::Box::new(t)))
    }
}

impl<T: TypstTypeLike> TypstTypeLike for Box<T> {
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        T::static_type_name()
    }
}


#[macro_export]
macro_rules! auto_impl {
    (
        $(#[derive($($meta:ty),*)])*
        $vis:vis enum $name:ident < $lt:lifetime > {
            $(
                #[try_from]
                $var_try:ident ( $ty_try:ty ),
            )*
            $(
                $var:ident ( $ty:ident$(<$($g:tt),*>)* )
            ),* $(,)?
        }
    ) => {
        $(#[derive($($meta),*)])*
        $vis enum $name<$lt> {
            $($var_try($ty_try),)*
            $($var($ty$(<$($g,)*>)*)),*
        }

        crate::impl_typst_type!(typst_like $name<$lt>, concat!($(stringify!($ty), "|", )* ">"));

        impl<$lt> TryFrom<crate::Item<$lt>> for $name<$lt> {
            type Error = std::string::String;

            fn try_from(value: crate::Item<$lt>) -> Result<Self, Self::Error> {
                match value {
                    $(
                        crate::Item::$ty(v) => Ok(Self::$var(v)),
                    )*
                    other => {
                        $(
                            if let Ok(v) = <$ty_try>::try_from(other.clone()) {
                                return Ok(Self::$var_try(v));
                            }
                        )*
                        Err(format!("Unable to cast Item into {}, found {:?}", stringify!($name), other))
                    }
                }
            }
        }

        impl<$lt> Into<crate::Item<$lt>> for $name<$lt> {
            fn into(self) -> crate::Item<$lt> {
                match self {
                    $(
                        $name::$var(v) => crate::Item::$ty(v),
                    )*
                    $(
                        $name::$var_try(v) => v.into(),
                    )*
                }
            }
        }

        impl<$lt, 'de: $lt> serde::Deserialize<'de> for $name<$lt>
        where
            $name<$lt>: TryFrom<crate::Item<$lt>, Error=std::string::String>,
        {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let item = crate::Item::deserialize(deserializer)?;
                $name::try_from(item).map_err(serde::de::Error::custom)
            }
        }

        impl<$lt> serde::Serialize for $name<$lt>
        where
            $name<$lt>: Clone + Into<crate::Item<$lt>>,
        {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let item: crate::Item = self.clone().into();
                item.serialize(serializer)
            }
        }
    };
}

#[macro_export]
macro_rules! auto_impl_str {
    (
        $(#[derive($($meta:ty),*)])*
        $vis:vis enum $name:ident {
            $(
                $(#[$var_meta:meta])*
                $var:ident = $str:expr
            ),* $(,)?
        }
    ) => {
        $(#[derive($($meta),*)])*
        #[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
        #[serde(try_from = "crate::Item", into = "crate::Item")]
        $vis enum $name {
            $(
                $(#[$var_meta])*
                $var,
            )*
        }

        crate::impl_typst_type!(typst_like $name, concat!($($str, "|", )*));

        impl<'a> TryFrom<crate::String<'a>> for $name {
            type Error = std::string::String;

            fn try_from(value: crate::String<'a>) -> Result<Self, Self::Error> {
                match &*value {
                    $(
                        $str => Ok(Self::$var),
                    )*
                    _ => Err(format!("Unable to cast String into {}, found {}", <Self as crate::TypstTypeLike>::static_type_name(), &*value)),
                }
            }
        }

        impl<'a> Into<crate::String<'a>> for $name {
            fn into(self) -> crate::String<'a> {
                match self {
                    $(
                        $name::$var => $str.into(),
                    )*
                }
            }
        }

        impl<'a> TryFrom<crate::Item<'a>> for $name {
            type Error = std::string::String;

            fn try_from(value: crate::Item<'a>) -> Result<Self, Self::Error> {
                match value {
                    crate::Item::String(s) => s.try_into(),
                    other => Err(format!("Unable to cast Item into String, found {:?}", other)),
                }
            }
        }

        impl<'a> Into<crate::Item<'a>> for $name {
            fn into(self) -> crate::Item<'a> {
                let s: crate::String<'a> = self.into();
                crate::Item::String(s)
            }
        }
    };
}

#[macro_export]
macro_rules! auto_impl_func {
    (
        $(#[derive($($meta:ty),*)])*
        $vis:vis enum $name:ident {
            $(
                $(#[$var_meta:meta])*
                $var:ident = $str:expr
            ),* $(,)?
        }
    ) => {
        $(#[derive($($meta),*)])*
        #[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
        #[serde(try_from = "crate::Function", into = "crate::Function")]
        $vis enum $name {
            $(
                $(#[$var_meta])*
                $var,
            )*
        }

        crate::impl_typst_type!(typst_like $name, concat!($($str, "|", )*));

        impl<'a> TryFrom<crate::Function<'a>> for $name {
            type Error = std::string::String;

            fn try_from(func: crate::Function<'a>) -> Result<Self, Self::Error> {
                match func.full_name().as_deref() {
                    $(
                        Some($str) => Ok(Self::$var),
                    )*
                    _ => Err(format!("Unable to cast Function into {}, found {:?}", <Self as crate::TypstTypeLike>::static_type_name(), func)),
                }
            }
        }

        impl<'a> Into<crate::Function<'a>> for $name {
            fn into(self) -> crate::Function<'a> {
                match self {
                    $(
                        $name::$var => crate::Function::Named($str.into()),
                    )*
                }
            }
        }
    };
}