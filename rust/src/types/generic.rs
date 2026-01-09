use crate::types::Item;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(from = "Item", into = "Item")]
pub enum Or<
    'a,
    T1: Clone + Into<Item<'a>> + TryFrom<Item<'a>>,
    T2: Clone + Into<Item<'a>> + TryFrom<Item<'a>>
> where
    <T1 as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
    <T2 as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
{
    Type1(T1),
    Type2(T2),
    #[serde(borrow)]
    __Phantom(&'a ()),
}

impl<
    'a,
    T1: Clone + TryFrom<Item<'a>> + Into<Item<'a>>,
    T2: Clone + TryFrom<Item<'a>> + Into<Item<'a>>
> From<Item<'a>> for Or<'a, T1, T2>
where
    <T1 as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
    <T2 as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
{
    fn from(value: Item<'a>) -> Self {
        T1::try_from(value.clone())
            .map(Or::Type1)
            .or_else(|_| T2::try_from(value).map(Or::Type2))
            .expect("Type did not match either variant in Or<T1, T2>")
    }
}

impl<
    'a,
    T1: Clone + TryFrom<Item<'a>> + Into<Item<'a>>,
    T2: Clone + TryFrom<Item<'a>> + Into<Item<'a>>
> From<Or<'a, T1, T2>> for Item<'a>
where
    <T1 as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
    <T2 as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
{
    fn from(value: Or<'a,T1, T2>) -> Self {
        match value {
            Or::Type1(t1) => t1.into(),
            Or::Type2(t2) => t2.into(),
            Or::__Phantom(_) => unreachable!(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(try_from = "Item", into = "Item")]
pub enum AutoOr<
    'a,
    T: Clone + Into<Item<'a>> + TryFrom<Item<'a>>,
> where
    <T as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
{
    Auto,
    Type1(T),
    #[serde(borrow)]
    __Phantom(&'a ()),
}

impl<
    'a,
    T: Clone + TryFrom<Item<'a>> + Into<Item<'a>>,
> TryFrom<Item<'a>> for AutoOr<'a, T>
where
    <T as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
{
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        match T::try_from(value.clone()).map(AutoOr::Type1) {
            Ok(v) => Ok(v),
            Err(_) => match value {
                Item::Auto => Ok(AutoOr::Auto),
                _ => Err("Type did not match either variant in AutoOr<T>".to_string()),
            },
        }
    }
}

impl<
    'a,
    T: Clone + TryFrom<Item<'a>> + Into<Item<'a>>,
> From<AutoOr<'a, T>> for Item<'a>
where
    <T as TryFrom<Item<'a>>>::Error: std::fmt::Debug,
{
    fn from(value: AutoOr<'a, T>) -> Self {
        match value {
            AutoOr::Type1(t1) => t1.into(),
            AutoOr::Auto => Item::Auto,
            AutoOr::__Phantom(_) => unreachable!(),
        }
    }
}

pub struct Result<'a, T>(std::result::Result<T, crate::types::String<'a>>);
impl<'a, T> From<std::result::Result<T, crate::types::String<'a>>> for Result<'a, T> {
    fn from(value: std::result::Result<T, crate::types::String<'a>>) -> Self {
        Result(value)
    }
}
impl<'a, T> Into<Item<'a>> for Result<'a, T>
where
    T: Into<Item<'a>>,
{
    fn into(self) -> Item<'a> {
        match self.0 {
            Ok(v) => v.into(),
            Err(e) => Item::Panic(e.into()),
        }
    }
}

