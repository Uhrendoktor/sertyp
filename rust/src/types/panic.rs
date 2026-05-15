#[cfg(feature = "content")]
use std::fmt::Debug;
use std::ops::Deref;

use crate::{
    Content, Item, Text, TypedItem,
    types::{RBox, string::String},
};

/// When deserialized in typst results in a panic with the given message.
/// [crate::Result] automatically converts into this type in case of an error.
///
/// For more information visit the typst documentation: [panic](https://typst.app/docs/reference/foundations/panic/)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Hash)]
pub struct Panic<'a> {
    #[serde(borrow, rename = "type")]
    pub ty: String<'a>,
    #[serde(borrow)]
    pub msg: TypedItem<RBox<Content<'a>>>,
}

impl<'a> Panic<'a> {
    pub fn display_msg(&self) -> String<'a> {
        match self.msg.deref().as_ref() {
            #[cfg(feature = "content")]
            Content::Text(text) => format!("{}", text.as_string()),
            _ => format!("{:?}", self.msg),
        }
        .into()
    }
}

crate::impl_all!(Item<'a>::Panic, Panic<'a>{'a}, "panic");

impl<'a> From<String<'a>> for Panic<'a> {
    fn from(value: String<'a>) -> Self {
        Content::Text(Text::from_string(value).into()).into()
    }
}

impl<'a> From<Content<'a>> for Panic<'a> {
    fn from(value: Content<'a>) -> Self {
        RBox::new(value).into()
    }
}

impl<'a> From<RBox<Content<'a>>> for Panic<'a> {
    fn from(value: RBox<Content<'a>>) -> Self {
        Panic {
            ty: String::from("Panic"),
            msg: value.into_inner().into(),
        }
    }
}
