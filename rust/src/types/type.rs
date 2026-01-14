use std::{borrow::Cow, ops::Deref};

use crate::types::string::String;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Type<'a>(
    #[serde(borrow)]
    pub String<'a>
);

crate::impl_all!(Type<'a>, "type");

impl<'a> Deref for Type<'a> {
    type Target = String<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait TypstType {
    fn static_type_name() -> Cow<'static, str>;

    fn type_name(&self) -> Cow<'static, str> {
        Self::static_type_name()
    }
}

impl<T: TypstType> TypstTypeLike for T {
    fn static_type_name() -> Cow<'static, str> {
        T::static_type_name()
    }
}

pub trait TypstTypeLike {
    fn static_type_name() -> Cow<'static, str>;

    fn type_name(&self) -> Cow<'static, str> {
        Self::static_type_name()
    }
}

#[macro_export]
macro_rules! impl_typst_type {
    ($variant:ident$(<$lt:lifetime>)*, $name:expr) => {
        impl$(<$lt>)* crate::TypstType for $variant$(<$lt>)* {
            fn static_type_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }
    };
    (typst_like $variant:ident$(<$lt:lifetime>)*, $name:expr) => {
        impl$(<$lt>)* crate::TypstTypeLike for $variant$(<$lt>)* {
            fn static_type_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }
    };
}