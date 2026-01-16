use crate::Item;

pub struct Result<'a, T>(pub std::result::Result<T, crate::types::String<'a>>);
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

impl<'a, T> Into<Item<'a>> for std::result::Result<T, crate::types::String<'a>>
where
    T: Into<Item<'a>>,
{
    fn into(self) -> Item<'a> {
        Result::<'_, T>::from(self).into()
    }
}
