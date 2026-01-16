mod string;
mod integer;

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

use crate::types::selector::Selector;
pub use crate::types::r#type::{TypstType, TypstTypeLike};
pub use crate::types::generic::{Result, AutoOr, Or, Box};
pub use crate::types::{auto::Auto, none::None, boolean::Boolean, bytes::Bytes, panic::Panic, alignment::Alignment, angle::Angle, arguments::Arguments, array::Array, color::Color, content::Content, datetime::Datetime, decimal::Decimal, dictionary::Dictionary, direction::Direction, duration::Duration, float::Float, fraction::Fraction, function::Function, gradient::Gradient, integer::Integer, label::Label, length::Length, module::Module, ratio::Ratio, regex::Regex, relative::Relative, string::String, stroke::Stroke, styles::Styles, symbol::Symbol, tiling::Tiling, r#type::Type, version::Version};

crate::define_enum!{
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
        Content(Content<'a>),
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
        Gradient(Gradient<'a>),
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

crate::impl_typst_type!(typst_like Item<'a>, "item");

#[macro_export]
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
            #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
            $(#[$meta])* 
            $vis enum [<$name __>]<$lt> {
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
            #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
            #[serde(untagged)]
            pub enum [<$name _>]<$lt> {
                #[serde(borrow)]
                Typed([<$name __>]<$lt>),
                $(
                    $(#[$metau])*
                    $varu$(($tyu))?
                ),*
            }
        }

        paste::paste!{
            #[derive(Clone, Debug)]
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
                                [<$name __>]::$var $((crate::define_enum!(@make_arg $ty, r)))? => $name::$var$((crate::define_enum!(@make_arg $ty, r.into())))?,
                            )*
                            $(
                                [<$name __>]::$varr1 $((crate::define_enum!(@make_arg $tyr1, r)))? => crate::define_enum!(
                                    @make_arg $($tyr2)?, 
                                    $name::$varr2(crate::define_enum!(@make_arg $($tyr1)?, r.into(), {Default::default()})), 
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
            impl<$lt> Into<[<$name _>]<$lt>> for $name<$lt> {
                fn into(self) -> [<$name _>]<$lt> {
                    match self {
                        $(
                            $name::$var $((crate::define_enum!(@make_arg $ty, r)))? => [<$name _>]::Typed([<$name __>]::$var$((crate::define_enum!(@make_arg $ty, r.into())))?),
                        )*
                        $(
                            #[allow(unused)]
                            $name::$varr2(r) => [<$name _>]::Typed([<$name __>]::$varr1$((crate::define_enum!(@make_arg $tyr1, r.into())))?),
                        )*
                        $(
                            $name::$varu(r) => [<$name _>]::$varu(r),
                        )*
                    }
                }
            }
        } 

        paste::paste!{
            #[derive(Clone, Debug)]
            pub struct [<Typed $name>]<T>(pub T);

            impl<$lt, T: TryFrom<$name<$lt>, Error=std::string::String>> TryFrom<$name<$lt>> for [<Typed $name>]<T> {
                type Error = std::string::String;

                fn try_from(value: $name<$lt>) -> std::result::Result<Self, Self::Error> {
                    let typed: T = value.try_into()?;
                    Ok([<Typed $name>](typed) )
                }
            }

            impl<$lt, T: Into<$name<$lt>>> Into<$name<$lt>> for [<Typed $name>]<T> {
                fn into(self) -> $name<$lt> {
                    self.0.into()
                }
            }

            impl<$lt, 'de: $lt, T: serde::Deserialize<'de> + TryFrom<$name<$lt>, Error=std::string::String>> serde::Deserialize<'de> for [<Typed $name>]<T> {
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

            impl<$lt, T: crate::TypstTypeLike> crate::TypstTypeLike for [<Typed $name>]<T> {
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
        }
    };
    (@make_arg $ty:ty, $r:tt $(, $else:tt)?) => { $r };
    (@make_arg $ty:ty, $r:expr $(, $else:tt)?) => { $r };
    (@make_arg , $r:tt, $else:expr) => { $else };
    (@make_arg , $r:expr, $else:expr) => { $else };
}


#[macro_export]
macro_rules! impl_try_from {
    ($variant:ident$(<$lt:lifetime>)*) => {
        impl<'a> TryFrom<crate::Item<'a>> for $variant$(<$lt>)* {
            type Error = std::string::String;

            fn try_from(value: crate::Item<'a>) -> Result<Self, Self::Error> {
                match value {
                    crate::Item::$variant(v) => Ok(v),
                    _ => Err(format!("Tried to cast Item to {}, found {:?}", stringify!($variant), value)),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into {
    ($variant:ident$(<$lt:lifetime>)*) => {
        impl<'a> Into<crate::Item<'a>> for $variant$(<$lt>)* {
            fn into(self) -> crate::Item<'a> {
                crate::Item::$variant(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_all {
    ($variant:ident$(<$lt:lifetime>)*, $name:expr) => {
        crate::impl_try_from!($variant$(<$lt>)*);
        crate::impl_into!($variant$(<$lt>)*);
        crate::impl_typst_type!($variant$(<$lt>)*, $name);
    };
}