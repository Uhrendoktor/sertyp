use wasm_minimal_protocol::*;
use sertyp::*;

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

/// Does a full cycle of deserialization and serialization for test purposes.
#[typst_func]
pub fn cycle<'a>(
    value: sertyp::Item::<'a>
) -> sertyp::Item<'a> {
    value.into()
}