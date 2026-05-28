use crate::types::{Item, Item_};
use std::{collections::HashMap, hash::Hash};

/// Pre-deserialization / post-serialization helper struct for [Dictionary]. You probably want to use [Dictionary] instead.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct Dictionary_<'a>(#[serde(borrow)] pub HashMap<&'a str, Item_<'a>>);

/// For more information visit the typst documentation: [dictionary](https://typst.app/docs/reference/foundations/dictionary/)
///
/// # Note
/// The rust representation is built upon a `HashMap<&'a str, Item<'a>>`.
///
/// # Example
/// Create a dictionary and insert a few entries using static keys:
///
/// ```rust
/// use sertyp::{Dictionary, Item, Integer};
/// let mut dict: Dictionary = Dictionary::default();
/// dict.insert("count", Integer::i32(3).into());
/// ```
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
#[serde(from = "Dictionary_", into = "Dictionary_")]
pub struct Dictionary<'a>(#[serde(borrow)] HashMap<&'a str, Item<'a>>);

crate::impl_all!(Item<'a>::Dictionary, Dictionary<'a>{'a}, "dictionary");

impl<'a> Hash for Dictionary<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (k, v) in &**self {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl<'a> From<Dictionary<'a>> for Dictionary_<'a> {
    fn from(value: Dictionary<'a>) -> Self {
        Dictionary_(value.0.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<'a> From<Dictionary_<'a>> for Dictionary<'a> {
    fn from(value: Dictionary_<'a>) -> Self {
        Dictionary(value.0.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<'a> std::ops::Deref for Dictionary<'a> {
    type Target = HashMap<&'a str, Item<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for Dictionary<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<HashMap<&'a str, Item<'a>>> for Dictionary<'a> {
    fn from(value: HashMap<&'a str, Item<'a>>) -> Self {
        Dictionary(value)
    }
}

#[macro_export]
macro_rules! auto_impl_dict {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident $block:tt
    ) => {
        auto_impl_dict! {
            $(#[$meta])*
            $vis struct $name ['a] $block
        }
    };
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident$(<$($g:tt),*>)? $([$($g2:tt),*])? {
            $(
                $visf:vis $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name$(<$($g),*>)? {
            $($visf $field: $ty),*
        }

        impl <$($($g)*,)? $($($g2),*)?> TryFrom<sertyp::Dictionary<'a>> for $name$(<$($g),*>)? {
            type Error = std::string::String;

            fn try_from(mut value: sertyp::Dictionary<'a>) -> std::result::Result<Self, Self::Error> {
                Ok(Self {
                    $(
                        $field: value.remove(stringify!($field))
                            .ok_or_else(|| format!("Key '{}' not found in dictionary", stringify!($field)).to_string())?
                            .try_into()?,
                    )*
                })
            }
        }

        impl <$($($g)*,)? $($($g2),*)?> From<$name$(<$($g),*>)?> for sertyp::Dictionary<'a> {
            fn from(value: $name$(<$($g),*>)?) -> Self {
                let mut dict = sertyp::Dictionary::default();
                $(
                    dict.insert(stringify!($field), sertyp::Item::from(value.$field));
                )*
                dict
            }
        }

        impl <$($($g)*,)? $($($g2),*)?> TryFrom<sertyp::Item<'a>> for $name$(<$($g),*>)? {
            type Error = <sertyp::Dictionary<'a> as TryFrom<sertyp::Item<'a>>>::Error;

            fn try_from(item: sertyp::Item<'a>) -> std::result::Result<Self, Self::Error> {
                let dict: sertyp::Dictionary<'a> = item.try_into()?;
                dict.try_into()
            }
        }

        impl <$($($g)*,)? $($($g2),*)?> From<$name$(<$($g),*>)?> for sertyp::Item<'a> {
            fn from(value: $name$(<$($g),*>)?) -> Self {
                let dict: sertyp::Dictionary<'a> = value.into();
                dict.into()
            }
        }
    };
}
