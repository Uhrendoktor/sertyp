mod or;
mod typed_array;

#[allow(unused_imports)]
pub use or::{AutoOr, NoneOr, Or};
// pub use result::Result;
pub use typed_array::TypedArray;

use std::fmt::Debug;

use crate::types::{Color, Dictionary, Gradient, Length, Stroke, Tiling};
crate::auto_impl! {
    #[derive(Clone, Debug)]
    pub enum FillColor<'a> {
        try_from{},
        Color(Color=>Color<'a>),
        Gradient(Gradient=>std::boxed::Box<Gradient<'a>>),
        Tiling(Tiling=>Tiling)
    }
}

crate::auto_impl! {
    #[derive(Clone, Debug)]
    pub enum StrokeColor<'a> {
        try_from{},
        Length(Length=>Length),
        Color(Color=>Color<'a>),
        Gradient(Gradient=>std::boxed::Box<Gradient<'a>>),
        Stroke(Stroke=>Stroke<'a>),
        Tiling(Tiling=>Tiling),
        Dictionary(Dictionary=>Dictionary<'a>),
    }
}

/// Shortcut for specifying a type that is one of multiple possible variants. If you only need two variants, consider using [Or].
///
/// # Syntax
/// ```
/// try_from {
///     $variant ( $type ),
///     ...
/// }
/// ```
/// For types that cannot directly be mapped from a single [crate::Item] variant but support [TryFrom<Item>] + [Into<Item>] conversion for [crate::Item].
/// This is most often required when a type could be parsed from multiple different [crate::Item] variants.
/// | Parameter | Description |
/// | --- | --- |
/// | $variant | The name within your enum. |
/// | $type | The type to convert to/from. |
///
/// ```
/// $variant ($item_variant => $type)
/// ...
/// ```
/// For types that implement [TryFrom]&[Into] for the type of a specifc [crate::Item] variant.
/// | Parameter | Description |
/// | --- | --- |
/// | $variant | The name within your enum. |
/// | $item_variant | The variant name of [crate::Item] to grab the value from |
/// | $type | The type within your enum. Must implement [TryFrom] and [Into] for the type in crate::Item::$item_variant |
///
/// # Example
/// ```
/// crate::auto_impl! {
///     #[derive(Debug, Clone)]
///     pub enum OneOfMultiple<'a> {
///         try_from{},
///         String(String=>String<'a>),
///         Integer(Integer=>Integer),
///         Contents(Array=>TypedArray<Content<'a>>)
///     }
/// }
/// ```
/// This will create an enum `OneOfMultiple` that represents either a [crate::String], [crate::Integer] or an typed array of [crate::Content].
#[macro_export]
macro_rules! auto_impl {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident $(<$lt:lifetime>)? {
            try_from {
               $($var_try:ident ( $ty_try:ty )),*$(,)?
            },
            $(
                $var:ident ($var2:ident => $ty:ty )
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap, derive_more::From, derive_more::TryInto)]
        #[unwrap(owned, ref, ref_mut)]
        #[try_unwrap(owned, ref, ref_mut)]
        $vis enum $name$(<$lt>)? {
            $($var_try($ty_try),)*
            $($var($ty)),*
        }

        $crate::impl_typst_type!(typst_like $name$(<$lt>)?{$($lt),*}, concat!($(stringify!($var2), "|", )* ">"));

        impl<'a> TryFrom<$crate::Item<'a>> for $name$(<$lt>)? {
            type Error = std::string::String;

            fn try_from(value: $crate::Item<'a>) -> std::result::Result<Self, Self::Error> {
                match value {
                    $(
                        $crate::Item::$var2(v) => Ok(Self::$var(v.try_into().map_err(|e| format!("{e}"))?)),
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

        impl<'a> From<$name$(<$lt>)?> for $crate::Item<'a> {
            fn from(val: $name$(<$lt>)?) -> $crate::Item<'a> {
                match val {
                    $(
                        $name::$var(v) => $crate::Item::$var2(v.into()),
                    )*
                    $(
                        $name::$var_try(v) => v.into(),
                    )*
                }
            }
        }

        impl<'a, 'de: 'a> serde::Deserialize<'de> for $name$(<$lt>)?
        where
            $name$(<$lt>)?: TryFrom<$crate::Item<'a>>,
            <$name$(<$lt>)? as TryFrom<$crate::Item<'a>>>::Error: std::fmt::Display,
        {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let item = $crate::Item::deserialize(deserializer)?;
                $name::try_from(item).map_err(serde::de::Error::custom)
            }
        }

        impl<'a> serde::Serialize for $name$(<$lt>)?
        where
            $name$(<$lt>)?: Clone + Into<$crate::Item<'a>>,
        {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let item: $crate::Item = self.clone().into();
                item.serialize(serializer)
            }
        }
    };
}

/// Shortcut for specifying types that are string constants. Usually Typst uses predefined typed enums (e.g. direction, alignment, etc.). Nevertheless, many content functions accept string constants as parameters instead. It is recommended to hardcode those variants as enums on the Rust side. This macro helps with the creation of those string-constant enums.
/// # Example
/// ```
/// crate::auto_impl_str! {
///     pub enum TextAlign {
///         Left = "left",
///         Center = "center",
///         Right = "right",
///         Justify = "justify",
///     }
/// }
/// ```
/// Non matching strings will result in a deserialization error instead of applications having to catch invalid strings at runtime.
#[macro_export]
macro_rules! auto_impl_str {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$var_meta:meta])*
                $var:ident = $str:expr
            ),* $(,)?
        }
    ) => {
        #[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
        #[serde(try_from = "crate::String", into = "crate::String")]
        #[unwrap(owned, ref, ref_mut)]
        #[try_unwrap(owned, ref, ref_mut)]
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$var_meta])*
                $var,
            )*
        }

        $crate::impl_typst_type!(typst_like $name{}, concat!($($str, "|", )*));

        impl<'a> TryFrom<$crate::String<'a>> for $name {
            type Error = std::string::String;

            fn try_from(value: $crate::String<'a>) -> Result<Self, Self::Error> {
                match &*value {
                    $(
                        $str => Ok(Self::$var),
                    )*
                    _ => Err(format!("Unable to cast String into {}, found {}", <Self as $crate::TypstTypeLike>::static_type_name(), &*value)),
                }
            }
        }

        impl<'a> From<$name> for $crate::String<'a> {
            fn from(val: $name) -> $crate::String<'a> {
                match val {
                    $(
                        $name::$var => $str.into(),
                    )*
                }
            }
        }

        impl<'a> TryFrom<$crate::Item<'a>> for $name {
            type Error = std::string::String;

            fn try_from(value: $crate::Item<'a>) -> Result<Self, Self::Error> {
                match value {
                    $crate::Item::String(s) => s.try_into(),
                    other => Err(format!("Unable to cast Item into String, found {:?}", other)),
                }
            }
        }

        impl<'a> From<$name> for $crate::Item<'a> {
            fn from(val: $name) -> $crate::Item<'a> {
                let s: $crate::String<'a> = val.into();
                $crate::Item::String(s)
            }
        }
    };
}

/// Shortcut for specifying function spaces. Typst types like `color` define a set of valid functions as parameters. This macro helps with the creation of those function-constant enums. Functions are serialized as namespace strings (e.g. `blend.normal`).
/// # Example
/// ```
/// crate::auto_impl_func! {
///     pub enum BlendMode {
///         Normal = "blend.normal",
///         Multiply = "blend.multiply",
///         Screen = "blend.screen",
///         Overlay = "blend.overlay",
///     }
/// }
/// ```
/// Non matching functions will result in a deserialization error instead of applications having to catch invalid functions at runtime.
#[macro_export]
macro_rules! auto_impl_func {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$var_meta:meta])*
                $var:ident = $str:expr
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
        #[serde(try_from = "crate::Function", into = "crate::Function")]
        $vis enum $name {
            $(
                $(#[$var_meta])*
                $var,
            )*
        }

        $crate::impl_typst_type!(typst_like $name{}, concat!($($str, "|", )*));

        impl<'a> TryFrom<$crate::Function<'a>> for $name {
            type Error = std::string::String;

            fn try_from(func: $crate::Function<'a>) -> Result<Self, Self::Error> {
                match func.full_name().as_deref() {
                    $(
                        Some($str) => Ok(Self::$var),
                    )*
                    _ => Err(format!("Unable to cast Function into {}, found {:?}", <Self as $crate::TypstTypeLike>::static_type_name(), func)),
                }
            }
        }

        impl<'a> From<$name> for $crate::Function<'a> {
            fn from(val: $name) -> $crate::Function<'a> {
                match val {
                    $(
                        $name::$var => $crate::Function::Named($str.into()),
                    )*
                }
            }
        }
    };
}
