use derive_more::{Display, IsVariant, TryUnwrap, Unwrap};

use crate::{Item, types::string::String};

/// A function.
/// This can either be
/// - An inline function, represented as `(..) => ..`
/// - A named function, represented by its full name as a string.
///
/// namespaces are separated by dots (`.`), e.g. `math.sin` or `document.create`.
/// More information than the functions namespace cannot be extracted from the typst type.
/// Note: A deserialized function in typst **is callable**.
///
/// For more information visit the typst documentation: [function](https://typst.app/docs/reference/foundations/function/)
#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    Hash,
    Display,
    IsVariant,
    Unwrap,
    TryUnwrap,
)]
#[serde(from = "String", into = "String")]
#[unwrap(owned, ref, ref_mut)]
#[try_unwrap(owned, ref, ref_mut)]
pub enum Function<'a> {
    #[default]
    #[display("(..) => ..")]
    Inline,
    #[serde(borrow)]
    #[display("{}", **_0)]
    Named(String<'a>),
}

crate::impl_all!(Item<'a>::Function, Function<'a>{'a}, "function");

impl<'a> From<String<'a>> for Function<'a> {
    fn from(value: String<'a>) -> Self {
        if &*value == "(..) => .." {
            Function::Inline
        } else {
            Function::Named(value)
        }
    }
}

impl<'a> From<Function<'a>> for String<'a> {
    fn from(val: Function<'a>) -> Self {
        match val {
            Function::Inline => "(..) => ..".into(),
            Function::Named(name) => name,
        }
    }
}

impl<'a> Function<'a> {
    /// Get the basename of the function (without namespace prefix).
    /// `math.sin` -> `sin`
    pub fn name(&'a self) -> Option<String<'a>> {
        match self {
            Function::Inline => None,
            Function::Named(name) => name.rsplit(".").next().map(|s| s.into()),
        }
    }

    /// Get the full name of the function (with namespace prefix).
    pub fn full_name(&'a self) -> Option<String<'a>> {
        match self {
            Function::Inline => None,
            Function::Named(name) => Some((&**name).into()),
        }
    }

    /// Get the namespace (context) of the function.
    /// `math.sin` -> `math`
    pub fn ctx_name(&'a self) -> Option<String<'a>> {
        match self {
            Function::Inline => None,
            Function::Named(name) => {
                let mut parts = name.rsplitn(2, ".");
                parts.next();
                parts.next().map(|s| s.into())
            }
        }
    }
}
