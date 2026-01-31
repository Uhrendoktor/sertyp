use crate::Item;

/// Utility type for returning results within typst WASM plugin functions. Returning an Result::Err in a plugin function will throw a runtime panic with the corresponding error message within typst.
/// # Example
/// ```
/// use sertyp::typst_func;
///
/// //#[typst_func]
/// fn somefn<'a>(_arg: sertyp::Integer) -> sertyp::Result<'a, sertyp::Integer> {
///     // some code that might fail
///     Err("an error occurred".into()).into()
/// }
/// ```
/// # Note
/// [crate::typst_func] also supports returning [std::result::Result]<T, [crate::types::String]> directly, enabling `?` syntax.
#[derive(Clone, Debug)]
pub struct Result<'a, T>(pub std::result::Result<T, crate::types::String<'a>>);

impl<'a, T> Default for Result<'a, T> {
    fn default() -> Self {
        Result(Err("default error".into()))
    }
}

impl<'a, T> From<std::result::Result<T, crate::types::String<'a>>> for Result<'a, T> {
    fn from(value: std::result::Result<T, crate::types::String<'a>>) -> Self {
        Result(value)
    }
}
impl<'a, T> From<Result<'a, T>> for Item<'a>
where
    T: Into<Item<'a>>,
{
    fn from(val: Result<'a, T>) -> Self {
        match val.0 {
            Ok(v) => v.into(),
            Err(e) => Item::Panic(e.into()),
        }
    }
}

impl<'a, T> From<std::result::Result<T, crate::types::String<'a>>> for Item<'a>
where
    T: Into<Item<'a>>,
{
    fn from(val: std::result::Result<T, crate::types::String<'a>>) -> Self {
        Result::<'_, T>::from(val).into()
    }
}
