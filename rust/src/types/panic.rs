use crate::{Item, types::string::String};

/// When deserialized in typst results in a panic with the given message.
/// [crate::Result] automatically converts into this type in case of an error.
///
/// For more information visit the typst documentation: [panic](https://typst.app/docs/reference/foundations/panic/)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct Panic<'a>(#[serde(borrow)] pub String<'a>);

crate::impl_all!(Item<'a>::Panic, Panic<'a>{'a}, "panic");

impl<'a> From<String<'a>> for Panic<'a> {
    fn from(value: String<'a>) -> Self {
        Panic(value)
    }
}
