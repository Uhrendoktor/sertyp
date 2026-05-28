use sertyp::typst_func;
use wasm_minimal_protocol::*;

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

/// Does a full cycle of deserialization and serialization for test purposes.
#[typst_func]
pub fn cycle<'a>(value: sertyp::Item<'a>) -> sertyp::Item<'a> {
    value
}

/// Does a full cycle of deserialization and serialization for test purposes.
#[typst_func]
pub fn multiarg(value: sertyp::Integer, other: sertyp::Integer) -> sertyp::Integer {
    sertyp::Integer::from(i32::try_from(value).unwrap_or(0) + i32::try_from(other).unwrap_or(0))
}

/// Dummy function that does not expect a sertyp::Panic as input and therefore cascades the error if it receives one.
#[typst_func]
pub fn not_expecting_error<'a>(value: sertyp::Content<'a>) -> sertyp::Content<'a> {
    value
}
