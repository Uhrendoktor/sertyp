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

#[typst_func]
pub fn fibonacci<'a>(
    n: Integer,
) -> sertyp::Result<'a, Integer> {
    let n: i32 = match n.try_into() {
        Ok(n) => n,
        Err(_) => return Err("Invalid integer range".into()).into()
    };
    
    let (mut v0, mut v1) = (0, 1);
    for _ in 0..n {
        (v0, v1) = (v1, v0 + v1);
    }

    Ok(v1.into()).into()
}