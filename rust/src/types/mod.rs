mod integer;
mod string;

mod generic;

mod alignment;
mod angle;
mod arguments;
mod array;
mod auto;
mod boolean;
mod bytes;
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
mod none;
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

pub use crate::types::generic::{AutoOr, Box as RBox, NoneOr, Or, Result, TypedArray};
use crate::types::selector::Selector;
pub use crate::types::r#type::{TypstType, TypstTypeLike};
pub use crate::types::{
    alignment::Alignment, angle::Angle, arguments::Arguments, array::*, auto::Auto,
    boolean::Boolean, bytes::Bytes, color::Color, content::*, datetime::Datetime, decimal::Decimal,
    dictionary::Dictionary, direction::Direction, duration::Duration, float::Float,
    fraction::Fraction, function::Function, gradient::Gradient, integer::Integer, label::Label,
    length::Length, module::Module, none::None, panic::Panic, ratio::Ratio, regex::Regex,
    relative::Relative, string::String, stroke::Stroke, styles::Styles, symbol::Symbol,
    tiling::Tiling, r#type::Type, version::Version,
};

crate::define_enum! {
    #[serde(tag = "type", content = "value", rename_all = "lowercase")]
    pub enum Item<'a> {
        untagged {
            Array(Array<'a>),
            Boolean(Boolean),
            Integer(Integer),
            String(String<'a>),
            Bytes(Bytes<'a>),
        },
        remap {
            Auto => Auto(Auto),
            None => None(None),
        },
        Alignment(Alignment),
        Angle(Angle),
        #[serde(borrow)]
        Arguments(Arguments<'a>),
        #[serde(borrow)]
        Color(Color<'a>),
        #[serde(borrow)]
        Content(ItemContent<'a>),
        Datetime(Datetime),
        #[serde(borrow)]
        Decimal(Decimal<'a>),
        #[serde(borrow)]
        Dictionary(Dictionary<'a>),
        Direction(Direction),
        Duration(Duration),
        Float(Float),
        Fraction(Fraction),
        #[serde(borrow)]
        Function(Function<'a>),
        #[serde(borrow)]
        Gradient(std::boxed::Box<Gradient<'a>>),
        #[serde(borrow)]
        Label(Label<'a>),
        Length(Length),
        #[serde(borrow)]
        Module(Module<'a>),
        Ratio(Ratio),
        Relative(Relative),
        #[serde(borrow)]
        Regex(Regex<'a>),
        Selector(Selector),
        #[serde(borrow)]
        Stroke(Stroke<'a>),
        #[serde(borrow)]
        Styles(Styles<'a>),
        #[serde(borrow)]
        Symbol(Symbol<'a>),
        Tiling(Tiling),
        #[serde(borrow)]
        Type(Type<'a>),
        Version(Version),

        #[serde(borrow)]
        Panic(Panic<'a>),
    }
}

crate::impl_typst_type!(typst_like Item<'a>{'a}, "item");

/// Default `Item` is an empty string.
impl<'a> Default for Item<'a> {
    fn default() -> Self {
        String::default().into()
    }
}

#[macro_export]
/// Generates all utilities required to serde serialize/deserialize an enum with fields that are `untagged` and `tagged` at the same time.
/// Furthermore type remapping is supported.
///
/// - untagged: variants that are untagged in the serialized format.
/// - remap: variants that cannot be deserialized directly, but require an intermediate representation. Mostly used to beauty uplift type definitions.
/// - ...: normal variants that adhere to serde's tagged enum format.
///
/// Since the types do not store information about their variant and thus cannot directly be deserialized, an additional Typed<enum_name> type is generated that can be used to directly deserialize into a specific enum variant.
///
/// # Example
/// ```rust
/// use sertyp::{define_enum, impl_try_from};
///
/// #[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug)]
/// pub struct Marker;
/// #[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug)]
/// pub struct Tagged<'a> (&'a str);
///
/// impl_try_from!(u8);
///
/// define_enum!{
///     #[serde(tag = "type", content = "value")]
///     pub enum Example<'a> {
///         untagged {
///             Untagged(u8),
///         },
///         remap {
///            Valueless => ValuelessWithMarker(Marker),
///         },   
///         #[serde(borrow, rename="tagged")]   
///         Tagged(Tagged<'a>),
///     }
/// }
///
/// fn test() {
///     // Primitive variant
///     serde_json::from_str::<Example>("42").unwrap();
///     // Valueless variant remapped to ValueslessWithMarker
///     let Example::ValuelessWithMarker(marker) =  serde_json::from_str::<Example>(r#"{"type": "Valueless"}"#).unwrap() else {
///         return;
///     };
///     // Using TypedExample to directly deserialize into the typed variant
///     let typed: TypedExample<u8> = serde_json::from_str("42").unwrap();
/// }
/// ```
macro_rules! define_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident<$lt:lifetime> {
            untagged {$(
                $(#[$metau:meta])*
                $varu:ident$(($tyu:ty))?
            ),*$(,)?},
            remap {$(
                $(#[$metar:meta])*
                $varr1:ident $(($tyr1:ty))? => $varr2:ident $(($tyr2:ty))?
            ),*$(,)?},
            $(
                $(#[$metav:meta])*
                $var:ident $(($ty:ty))?
            ),*$(,)?
        }
    ) => {
        paste::paste!{
            #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
            $(#[$meta])*
            enum [<$name __>]<$lt> {
                $(
                    $(#[$metav])*
                    $var $(($ty))?,
                )*
                $(
                    $(#[$metar])*
                    $varr1 $(($tyr1))?,
                )*
            }
        }

        paste::paste!{
            #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
            #[serde(untagged)]
            enum [<$name _>]<$lt> {
                #[serde(borrow)]
                Typed([<$name __>]<$lt>),
                $(
                    $(#[$metau])*
                    $varu$(($tyu))?
                ),*
            }
        }

        paste::paste!{
            #[derive(Clone, Debug, Hash)]
            pub enum $name<$lt> {
                $(
                    $var$(($ty))?,
                )*
                $(
                    $varr2$(($tyr2))?,
                )*
                $(
                    $varu$(($tyu))?
                ),*
            }
        }

        paste::paste!{
            impl<'de: $lt, $lt> serde::Deserialize<'de> for $name<$lt> {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let intermediate = [<$name _>]::deserialize(deserializer)?;
                    Ok(intermediate.into())
                }
            }
        }

        paste::paste!{
            impl<$lt> serde::Serialize for $name<$lt> {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let intermediate: [<$name _>] = self.clone().into();
                    intermediate.serialize(serializer)
                }
            }
        }

        paste::paste!{
            impl<$lt> From<[<$name _>]<$lt>> for $name<$lt> {
                fn from(value: [<$name _>]<$lt>) -> Self {
                    match value {
                        [<$name _>]::Typed(t) => match t {
                            $(
                                [<$name __>]::$var $(($crate::if_else!($ty, r)))? => $name::$var$(($crate::if_else!($ty, r.into())))?,
                            )*
                            $(
                                [<$name __>]::$varr1 $(($crate::if_else!($tyr1, r)))? => $crate::if_else!(
                                    $($tyr2)?,
                                    $name::$varr2($crate::if_else!($($tyr1)?, r.into(), {Default::default()})),
                                    {$name::$varr2}
                                ),
                            )*
                        },
                        $(
                            [<$name _>]::$varu(r) => $name::$varu(r),
                        )*
                    }
                }
            }
        }

        paste::paste!{
            impl<$lt> From<$name<$lt>> for [<$name _>]<$lt> {
                fn from(v: $name<$lt>) -> [<$name _>]<$lt> {
                    match v {
                        $(
                            $name::$var $(($crate::if_else!($ty, r)))? => [<$name _>]::Typed([<$name __>]::$var$(($crate::if_else!($ty, r.into())))?),
                        )*
                        $(
                            #[allow(unused)]
                            $name::$varr2(r) => [<$name _>]::Typed([<$name __>]::$varr1$(($crate::if_else!($tyr1, r.into())))?),
                        )*
                        $(
                            $name::$varu(r) => [<$name _>]::$varu(r),
                        )*
                    }
                }
            }
        }

        paste::paste!{
            #[derive(Clone, Debug, Default, Hash)]
            pub struct [<Typed $name>]<T>(pub T);

            impl<$lt, T: TryFrom<$name<$lt>>> TryFrom<$name<$lt>> for [<Typed $name>]<T> {
                type Error = T::Error;

                fn try_from(value: $name<$lt>) -> std::result::Result<Self, Self::Error> {
                    let typed: T = value.try_into()?;
                    Ok([<Typed $name>](typed))
                }
            }

            impl<$lt, T: Into<$name<$lt>>> From<[<Typed $name>]<T>> for $name<$lt> {
                fn from(value: [<Typed $name>]<T>) -> $name<$lt> {
                    value.0.into()
                }
            }

            impl<$lt, 'de: $lt, T: TryFrom<$name<$lt>>> serde::Deserialize<'de> for [<Typed $name>]<T>
            where
                T::Error: std::fmt::Display
            {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let intermediate = $name::<$lt>::deserialize(deserializer)?;
                    let typed: T = intermediate.try_into().map_err(serde::de::Error::custom)?;
                    Ok([<Typed $name>](typed))
                }
            }

            impl<$lt, T: Clone + Into<$name<$lt>>> serde::Serialize for [<Typed $name>]<T> {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let intermediate: $name<$lt> = self.0.clone().into();
                    intermediate.serialize(serializer)
                }
            }

            impl<$lt, T: $crate::TypstTypeLike> $crate::TypstTypeLike for [<Typed $name>]<T> {
                fn static_type_name() -> std::borrow::Cow<'static, str> {
                    T::static_type_name()
                }
            }

            impl<T> std::ops::Deref for [<Typed $name>]<T> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T> std::ops::DerefMut for [<Typed $name>]<T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<T> [<Typed $name>]<T> {
                /// Consumes the typed wrapper and returns the inner value.
                pub fn into_inner(self) -> T {
                    self.0
                }

                pub fn new(value: T) -> Self {
                    [<Typed $name>](value)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! if_else {
    ($cond:ty, $if:tt $(, $else:tt)?) => {
        $if
    };
    ($cond:ty, $if:expr $(, $else:tt)?) => {
        $if
    };
    ($cond:tt, $if:tt $(, $else:tt)?) => {
        $if
    };
    ($cond:tt, $if:expr $(, $else:tt)?) => {
        $if
    };
    (, $if:tt, $else:expr) => {
        $else
    };
    (, $if:expr, $else:expr) => {
        $else
    };
    (, $if:tt, $else:tt) => {
        $else
    };
    (, $if:expr, $else:tt) => {
        $else
    };
}

#[macro_export]
/// Implement `TryFrom<Item<'a>>` for a variant type.
macro_rules! impl_try_from {
    ($fty:ident$(<$flt:lifetime>)?::$variant:ident, $ty:ty) => {
        paste::paste!{impl<'a> TryFrom<$fty$(<$flt>)?> for $ty {
            type Error = std::string::String;

            fn try_from(value: $fty$(<$flt>)?) -> Result<Self, Self::Error> {
                match value {
                    $fty::$(<$flt>)?::$variant(v) => Ok(v),
                    _ => Err(format!("Tried to cast Item to {}, found {:?}", stringify!($ty), value)),
                }
            }
        }}
    };
}

#[macro_export]
/// Implement `Into<Item<'a>>` for a variant type.
macro_rules! impl_into {
    ($ity:ident$(<$ilt:lifetime>)?::$variant:ident, $ty:ty) => {
        impl<'a> From<$ty> for $ity$(<$ilt>)? {
            fn from(value: $ty) -> $ity$(<$ilt>)? {
                $ity::$(<$ilt>)?::$variant(value)
            }
        }
    };
}

#[macro_export]
/// Implement utility conversion for Typed`enum_name`<T> types.
/// T -> Typed`enum_name`<T>
/// T -> Typed`enum_name`<Box<T>>
/// T -> Typed`enum_name`<std::boxed::Box<T>>
/// T -> Typed`enum_name`<std::option::Option<T>>
macro_rules! impl_into_typed {
    ($ity:ident, $ty:ty) => {
        paste::paste! {impl<'a> From<$ty> for $crate::[<Typed $ity>]<$ty> {
            fn from(value: $ty) -> $crate::[<Typed $ity>]<$ty> {
                $crate::[<Typed $ity>]::new(value)
            }
        }}

        paste::paste! {impl<'a> From<$ty> for $crate::[<Typed $ity>]<$crate::RBox<$ty>> {
            fn from(value: $ty) -> $crate::[<Typed $ity>]<$crate::RBox<$ty>> {
                $crate::[<Typed $ity>]::new(value.into())
            }
        }}

        paste::paste! {impl<'a> From<$ty> for $crate::[<Typed $ity>]<std::boxed::Box<$ty>> {
            fn from(value: $ty) -> $crate::[<Typed $ity>]<std::boxed::Box<$ty>> {
                $crate::[<Typed $ity>]::new(value.into())
            }
        }}

        paste::paste! {impl<'a> From<$ty> for $crate::[<Typed $ity>]<std::option::Option<$ty>> {
            fn from(value: $ty) -> $crate::[<Typed $ity>]<std::option::Option<$ty>> {
                $crate::[<Typed $ity>]::new(Some(value))
            }
        }}
    };
}

#[macro_export]
/// Implement `TryFrom<Item<'a>>`, `Into<Item<'a>>`, and `TypstTypeLike`.
macro_rules! impl_all {
    ($sty:ident$(<$slt:lifetime>)?::$variant:ident, $ty:ty{$($g:tt),*}, $name:expr) => {
        $crate::impl_try_from!($sty$(<$slt>)?::$variant, $ty);
        $crate::impl_into!($sty$(<$slt>)?::$variant, $ty);
        $crate::impl_typst_type!($ty{$($g),*}, $name);
        $crate::impl_into_typed!($sty, $ty);
    };
    (typst_like $sty:ident$(<$slt:lifetime>)?::$variant:ident, $ty:ty{$($g:tt),*}, $name:expr) => {
        $crate::impl_try_from!($sty$(<$slt>)?::$variant, $ty);
        $crate::impl_into!($sty$(<$slt>)?::$variant, $ty);
        $crate::impl_typst_type!(typst_like $ty{$($g),*}, $name);
        $crate::impl_into_typed!($sty, $ty);
    };
}
