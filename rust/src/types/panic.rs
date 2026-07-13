#[cfg(not(feature = "content"))]
use std::collections::HashMap;
#[cfg(feature = "content")]
use std::fmt::Debug;

#[cfg(not(feature = "content"))]
use crate::FromString;
#[cfg(feature = "content")]
use crate::Text;
use crate::{Content, Item, ItemContent, TypedItem, types::string::String};

/// When deserialized in typst results in a panic with the given message.
/// [crate::Result] automatically converts into this type in case of an error.
///
/// For more information visit the typst documentation: [panic](https://typst.app/docs/reference/foundations/panic/)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Panic<'a> {
    #[serde(borrow, rename = "type")]
    pub ty: TypedItem<ItemContent<'a>>,
    #[serde(borrow)]
    #[cfg(feature = "content")]
    pub msg: TypedItem<Box<Content<'a>>>,
    #[cfg(not(feature = "content"))]
    pub msg: TypedItem<Content<'a>>,
}

crate::impl_all!(Item<'a>::Panic, Panic<'a>{'a}, "panic");

impl<'a> From<String<'a>> for Panic<'a> {
    fn from(value: String<'a>) -> Self {
        #[cfg(feature = "content")]
        return Content::Text(Text::from_string(value).into()).into();
        #[cfg(not(feature = "content"))]
        return Content {
            func: String::from("text").into(),
            fields: Some(
                vec![("text".into(), TypedItem(value))]
                    .into_iter()
                    .collect::<HashMap<_, _>>()
                    .into(),
            ),
        }
        .into();
    }
}

impl<'a> From<Content<'a>> for Panic<'a> {
    fn from(value: Content<'a>) -> Self {
        #[cfg(feature = "content")]
        return Box::new(value).into();
        #[cfg(not(feature = "content"))]
        return Panic {
            ty: ItemContent::from_string(String::from("Panic")).into(),
            msg: value.into(),
        };
    }
}

impl<'a> From<Box<Content<'a>>> for Panic<'a> {
    fn from(value: Box<Content<'a>>) -> Self {
        #[cfg(feature = "content")]
        return Panic {
            ty: Box::<Content>::new("Panic".into()).into(),
            msg: value.into(),
        };
        #[cfg(not(feature = "content"))]
        return (*value).into();
    }
}
