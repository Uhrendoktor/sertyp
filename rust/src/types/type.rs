use std::{borrow::Cow, ops::Deref};

use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [type](https://typst.app/docs/reference/foundations/type/)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default, Hash)]
pub struct Type<'a>(#[serde(borrow)] pub String<'a>);

crate::impl_all!(Item<'a>::Type, Type<'a>{'a}, "type");

impl<'a> Deref for Type<'a> {
    type Target = String<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Marker trait for all "real" typst types. For more information see [TypstTypeLike].
pub trait TypstType: TypstTypeLike {}

/// Trait for getting a typst-like type-name. This is mostly used for debug information. For "real" typst types (those who also implement [TypstType]) the value of [TypstTypeLike::static_type_name] corresponds to the correct typst type name as used in the sertyp cbor format.
pub trait TypstTypeLike {
    fn static_type_name() -> Cow<'static, str>;

    fn type_name(&self) -> Cow<'static, str> {
        Self::static_type_name()
    }
}

#[macro_export]
macro_rules! impl_typst_type {
    ($ty:ty{$($g:tt),*}, $name:expr) => {
        impl<$($g),*> $crate::TypstType for $ty {}
        impl<$($g),*> $crate::TypstTypeLike for $ty {
            fn static_type_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }
    };
    (typst_like $ty:ty{$($g:tt),*}, $name:expr) => {
        impl<$($g),*> $crate::TypstTypeLike for $ty {
            fn static_type_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }
    };
}
