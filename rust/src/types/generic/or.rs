use serde::{Deserialize, Serialize};

use crate::{Item, TypstTypeLike};

#[derive(Clone, Debug)]
pub enum Or<
    T1,
    T2
> {
    Left(T1),
    Right(T2),
}

impl<T1, T2> Default for Or<T1, T2>
where
    T1: Default,
{
    fn default() -> Self {
        Or::Left(T1::default())
    }
}

impl<'a, 'de: 'a, T1, T2> Deserialize<'de> for Or<T1, T2>
where
    Or<T1, T2>: TryFrom<Item<'a>, Error=std::string::String>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item = Item::deserialize(deserializer)?;
        Or::try_from(item).map_err(serde::de::Error::custom)
    }
}

impl<'a, T1: Clone + Into<Item<'a>>, T2: Clone + Into<Item<'a>>> Serialize for Or<T1, T2>
where
    Or<T1, T2>: Into<Item<'a>>,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let item: Item = self.clone().into();
        item.serialize(serializer)
    }
}

impl<
    'a,
    T1: Clone + TryFrom<Item<'a>, Error=std::string::String> + TypstTypeLike,
    T2: Clone + TryFrom<Item<'a>, Error=std::string::String> + TypstTypeLike,
> TryFrom<Item<'a>> for Or<T1, T2> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        let value_typ = <_ as TypstTypeLike>::type_name(&value);
        T1::try_from(value.clone()).map(Or::Left)
            .or_else(|_| T2::try_from(value).map(Or::Right))
            .map_err(|_| format!(
                "Type was expected to be {}, found {}",
                <Self as TypstTypeLike>::static_type_name(),
                value_typ
            ).into())
    }
}

impl<
    'a,
    T1: Into<Item<'a>>,
    T2: Into<Item<'a>>,
> Into<Item<'a>> for Or<T1, T2> {
    fn into(self) -> Item<'a> {
        match self {
            Or::Left(t1) => t1.into(),
            Or::Right(t2) => t2.into(),
        }
    }
}

impl<T1, T2> TypstTypeLike for Or<T1, T2>
where
    T1: TypstTypeLike,
    T2: TypstTypeLike,
{
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("{}|{}", T1::static_type_name(), T2::static_type_name()))
    }
}