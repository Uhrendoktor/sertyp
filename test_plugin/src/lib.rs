use wasm_minimal_protocol::*;
use sertyp::*;

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

/// Does a full cycle of deserialization and serialization for test purposes.
#[wasm_func]
pub fn cycle(
    data: &[u8],
) -> Vec<u8> {
    let err;
    let ty = match deserialize_cbor(data) {
        Ok(t) => t,
        Err(e) => {
            err = "Deserialization Error: ".to_string() + &e.to_string();
            Item::String(err.as_str().into())
        }
    };
    match serialize_cbor(&ty) {
        Ok(v) => v,
        Err(e) => {
            let err = "Serialization Error: ".to_string() + &e.to_string();
            serialize_cbor(&Item::String(err.as_str().into())).unwrap()
        }
    }       
}