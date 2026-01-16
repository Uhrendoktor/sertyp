use std::{fmt::Display, ops::Deref};

use crate::TypstType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct String<'a> (
    pub std::borrow::Cow<'a, str>
);
crate::impl_all!(String<'a>, "string");

impl<'a, 'de: 'a> serde::Deserialize<'de> for String<'a> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringVisitor<'a>(std::marker::PhantomData<&'a ()>);

        impl<'a, 'de: 'a> serde::de::Visitor<'de> for StringVisitor<'a> {
            type Value = String<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(<String as TypstType>::static_type_name().as_ref())
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


impl Deref for String<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a str> for String<'a> {
    fn from(value: &'a str) -> Self {
        String(std::borrow::Cow::Borrowed(value))
    }
}

impl<'a> From<std::string::String> for String<'a> {
    fn from(value: std::string::String) -> Self {
        String(std::borrow::Cow::Owned(value))
    }
}

impl<'a> From<std::borrow::Cow<'a, str>> for String<'a> {
    fn from(value: std::borrow::Cow<'a, str>) -> Self {
        String(value)
    }
}

impl<'a> Display for String<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}