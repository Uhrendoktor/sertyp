use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

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
#[derive(Clone, Debug, Deref, DerefMut, From, Into, AsRef, AsMut)]
pub struct Result<'a, T>(pub std::result::Result<T, crate::types::String<'a>>);

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
