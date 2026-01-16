use crate::{Auto, Item, Or, TypstTypeLike};

#[derive(Clone, Debug)]
pub enum AutoOr<
    T
> {
    Auto,
    Type(T),
}

impl<'a, 'de: 'a, T> serde::Deserialize<'de> for AutoOr<T>
where
    Self: 'a,
    AutoOr<T>: TryFrom<Item<'a>, Error=std::string::String>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item = Item::deserialize(deserializer)?;
        AutoOr::try_from(item).map_err(serde::de::Error::custom)
    }
}

impl<'a, T: Clone> serde::Serialize for AutoOr<T>
where
    AutoOr<T>: Into<Item<'a>>,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let item: Item = self.clone().into();
        item.serialize(serializer)
    }
}

impl<T> From<Or<Auto, T>> for AutoOr<T> {
    fn from(value: Or<Auto, T>) -> Self {
        match value {
            Or::Right(t) => AutoOr::Type(t),
            Or::Left(_) => AutoOr::Auto,
        }
    }
}

impl<T> Into<Or<Auto, T>> for AutoOr<T> {
    fn into(self) -> Or<Auto, T> {
        match self {
            AutoOr::Type(t) => Or::Right(t),
            AutoOr::Auto => Or::Left(Auto),
        }
    }
}

impl<'a, T> TryFrom<Item<'a>> for AutoOr<T>
where
    Or<Auto, T>: TryFrom<Item<'a>, Error=std::string::String>,
{
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> std::result::Result<Self, Self::Error> {
        let or: Or::<Auto, T> = value.try_into()?;
        Ok(AutoOr::from(or))
    }
}

impl<'a, T> Into<Item<'a>> for AutoOr<T>
where
    Or<Auto, T>: Into<Item<'a>>,
{
    fn into(self) -> Item<'a> {
        let or: Or<Auto, T> = self.into();
        or.into()
    }
}

impl<T> TypstTypeLike for AutoOr<T>
where
    T: TypstTypeLike,
{
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        <Or<Auto, T>>::static_type_name()
    }
}