use std::hash::Hash;

use derive_more::{Deref, DerefMut, Display};

use crate::{Item, TypstTypeLike};

/// When deserialized strings are represented as zero copy string slices.
/// When constructing values, owned strings can be used as well.
///
/// For more information visit the typst documentation: [string](https://typst.app/docs/reference/foundations/str/)
///
/// # Example
///
/// Convert a Rust `&str` into a sertyp `String` and into an `Item`:
///
/// ```rust
/// use sertyp::{String as String, Item};
/// let s: String = "hello".into();
/// let item: Item = s.into();
/// ```
#[derive(Clone, Debug, Eq, Deref, DerefMut, Display, Hash)]
#[deref(forward)]
#[deref_mut(forward)]
#[display("{}", **self)]
pub struct String<'a>(pub std::borrow::Cow<'a, str>);
crate::impl_all!(Item<'a>::String, String<'a>{'a}, "string");

impl<'a> Default for String<'a> {
    fn default() -> Self {
        String(std::borrow::Cow::Borrowed(""))
    }
}
impl<'a> PartialEq for String<'a> {
    fn eq(&self, other: &Self) -> bool {
        let s1: &str = self;
        let s2: &str = other;
        s1 == s2
    }
}

impl<'a, 'de: 'a> serde::Deserialize<'de> for String<'a> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringVisitor<'a>(std::marker::PhantomData<&'a ()>);

        impl<'a, 'de: 'a> serde::de::Visitor<'de> for StringVisitor<'a> {
            type Value = String<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(<String as TypstTypeLike>::static_type_name().as_ref())
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(String(v.into()))
            }
        }

        deserializer.deserialize_any(StringVisitor(std::marker::PhantomData))
    }
}

impl<'a> serde::Serialize for String<'a> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'a, 'b: 'a> From<&'b str> for String<'a> {
    fn from(value: &'b str) -> Self {
        String(std::borrow::Cow::Borrowed(value))
    }
}

impl<'a> From<std::string::String> for String<'a> {
    fn from(value: std::string::String) -> Self {
        String(std::borrow::Cow::Owned(value))
    }
}

impl<'a> From<String<'a>> for std::string::String {
    fn from(value: String<'a>) -> Self {
        value.0.into_owned()
    }
}

impl<'a> From<std::borrow::Cow<'a, str>> for String<'a> {
    fn from(value: std::borrow::Cow<'a, str>) -> Self {
        String(value)
    }
}
