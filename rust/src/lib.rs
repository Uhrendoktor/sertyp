mod types;
pub use types::*;

pub fn deserialize_cbor(
    data: &[u8]
) -> serde_cbor::Result<Item<'_>> {
    serde_cbor::from_slice::<Item<'_>>(data)
}

pub fn serialize_cbor(
    ty: &Item<'_>
) -> serde_cbor::Result<Vec<u8>> {
    serde_cbor::to_vec(&ty)
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let err = format!($($arg)*);
        let p: sertyp::Panic = err.as_str().into();
        return sertyp::serialize_cbor(&p.into()).unwrap();
    }};
}