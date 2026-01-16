mod or;
mod auto_or;
mod result;
mod r#box;
mod typed_array;

pub use or::Or;
pub use auto_or::AutoOr;
pub use result::Result;
pub use r#box::Box;
pub use typed_array::TypedArray;

use std::fmt::Debug;

use crate::types::{Color, Gradient, Tiling, Length, Dictionary, Stroke};
crate::auto_impl!{
    #[derive(Clone, Debug)]
    pub enum FillColor<'a> {
        Color(Color<'a>),
        Gradient(Gradient<'a>),
        Tiling(Tiling)
    }
}

crate::auto_impl!{
    #[derive(Clone, Debug)]
    pub enum StrokeColor<'a> {
        Length(Length),
        Color(Color<'a>),
        Gradient(Gradient<'a>),
        Stroke(Stroke<'a>),
        Tiling(Tiling),
        Dictionary(Dictionary<'a>),
    }
}


#[macro_export]
macro_rules! auto_impl {
    (
        $(#[derive($($meta:ty),*)])*
        $vis:vis enum $name:ident $(<$lt:lifetime>)? {
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
        $vis enum $name$(<$lt>)? {
            $($var_try($ty_try),)*
            $($var($ty$(<$($g,)*>)*)),*
        }

        crate::impl_typst_type!(typst_like $name$(<$lt>)?, concat!($(stringify!($ty), "|", )* ">"));

        impl<'a> TryFrom<crate::Item<'a>> for $name$(<$lt>)? {
            type Error = std::string::String;

            fn try_from(value: crate::Item<'a>) -> std::result::Result<Self, Self::Error> {
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

        impl<'a> Into<crate::Item<'a>> for $name$(<$lt>)? {
            fn into(self) -> crate::Item<'a> {
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

        impl<'a, 'de: 'a> serde::Deserialize<'de> for $name$(<$lt>)?
        where
            $name$(<$lt>)?: TryFrom<crate::Item<'a>, Error=std::string::String>,
        {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let item = crate::Item::deserialize(deserializer)?;
                $name::try_from(item).map_err(serde::de::Error::custom)
            }
        }

        impl<'a> serde::Serialize for $name$(<$lt>)?
        where
            $name$(<$lt>)?: Clone + Into<crate::Item<'a>>,
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
    (@lt_or_a, $lt:lifetime) => {
        <$lt>
    };
    (@lt_or_a, ) => {
        'a
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