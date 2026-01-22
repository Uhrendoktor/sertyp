use std::{fmt::{Debug, Display}, ops::Deref};

use crate::TypstType;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Bytes<'a> (
    pub std::borrow::Cow<'a, [u8]>
);
crate::impl_all!(Bytes<'a>, "bytes");


impl<'a, 'de: 'a> serde::Deserialize<'de> for Bytes<'a> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BytesVisitor<'a>(std::marker::PhantomData<&'a ()>);

        impl<'a, 'de: 'a> serde::de::Visitor<'de> for BytesVisitor<'a> {
            type Value = Bytes<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(<Bytes as TypstType>::static_type_name().as_ref())
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Bytes(v.into()))
            }
        }

        deserializer.deserialize_any(BytesVisitor(std::marker::PhantomData))
    }
}


impl<'a> serde::Serialize for Bytes<'a> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}


impl Deref for Bytes<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(value: &'a [u8]) -> Self {
        Bytes(std::borrow::Cow::Borrowed(value))
    }
}

impl<'a> From<std::vec::Vec<u8>> for Bytes<'a> {
    fn from(value: std::vec::Vec<u8>) -> Self {
        Bytes(std::borrow::Cow::Owned(value))
    }
}

impl<'a> From<std::borrow::Cow<'a, [u8]>> for Bytes<'a> {
    fn from(value: std::borrow::Cow<'a, [u8]>) -> Self {
        Bytes(value)
    }
}

impl<'a> Display for Bytes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}