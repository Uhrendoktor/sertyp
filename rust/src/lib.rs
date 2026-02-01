/// # Sertyp - Rust Backend
///
/// Rust data structures for Typst values with serialization and deserialization
/// from and into the sertyp CBOR format. Allows for straightforward communication
/// between Typst and WASM plugins.
///
/// ## Overview
///
/// This library provides serialization and deserialization logic for the **sertyp**
/// CBOR format into handy Rust data structures and utility functions. Instead of
/// manually parsing untyped bytes or strings, you receive proper Rust types with
/// structured access to components.
///
/// ## Supported Types
///
/// - **Primitives**: `bool`, `int`, `float`, `string`, `bytes`, `none`, `auto`
/// - **Common**: `array`, `dict`, `function`, `type`, `decimal`
/// - **Typst-specific**: `alignment`, `angle`, `color`, `length`, `relative`,
///   `ratio`, `fraction`, `duration`, `datetime`, `symbol`, `label`, `regex`,
///   `stroke`, `gradient`, `tiling`, `direction`, `version`, `module`, `styles`,
///   `content`
/// - **Errors**: `panic`
/// - **Content**: using the `content` feature flag, typed content structures like `math.equation`, `text`, `math.mat`, `math.vec`, ... are supported.
///
/// ## Example
///
/// ```rust
/// use wasm_minimal_protocol::*;
/// use sertyp::{typst_func, Integer, String};
///
/// #[cfg(target_arch = "wasm32")]
/// initiate_protocol!();
///
/// // Create a plugin function that receives typed Typst values
/// #[typst_func]
/// pub fn fibonacci<'a>(n: Integer) -> Result<Integer, String<'a>> {
///     let n: i32 = n.try_into().map_err(|_| "Invalid integer range")?;
///
///     let (mut v0, mut v1) = (0, 1);
///     for _ in 0..n {
///         (v0, v1) = (v1, v0 + v1);
///     }
///
///     Ok(v1.into())
/// }
/// ```
///
/// From Typst:
/// ```typst
/// #import "@preview/sertyp:0.1.2"
/// #let plugin = plugin("<...>.wasm")
/// #let result = sertyp.call(plugin.fibonacci, 10)
/// #assert(result == 89)
/// ```
mod types;
pub use sertyp_macros::*;
pub use types::*;

/// Deserialize CBOR bytes into a sertyp [Item].
///
/// Takes raw CBOR-encoded bytes (typically received from Typst via WASM) and
/// deserializes them into a strongly-typed [Item] enum, which can then be
/// converted into specific Typst types like [Color], [Length], [Integer], etc.
///
/// # Arguments
///
/// * `data` - CBOR-encoded bytes representing a serialized Typst value
///
/// # Returns
///
/// Returns a [std::result::Result] containing the deserialized [Item] or a deserialization error.
///
/// # Example
///
/// ```rust
/// use sertyp::{deserialize_cbor, Item, error};
///
/// fn example(cbor_bytes: &[u8]) -> Vec<u8> {
///     match deserialize_cbor(cbor_bytes) {
///         Ok(Item::Integer(i)) => println!("Received integer: {}", i),
///         Ok(Item::String(s)) => println!("Received string: {}", s),
///         Ok(_) => println!("Received some other type"),        
///         Err(e) => error!("{e}"),      
///     }
///     vec![]
/// }
/// ```
pub fn deserialize_cbor(data: &[u8]) -> serde_cbor::Result<Item<'_>> {
    serde_cbor::from_slice::<Item<'_>>(data)
}

/// Serialize a sertyp [Item] into CBOR bytes.
///
/// Converts a strongly-typed [Item] into CBOR-encoded bytes that can be sent
/// back to Typst from a WASM plugin. The resulting bytes can be deserialized
/// on the Typst side using `sertyp.deserialize-cbor()`.
///
/// # Arguments
///
/// * `ty` - A reference to the [Item] to serialize
///
/// # Returns
///
/// Returns a [std::result::Result] containing the CBOR-encoded bytes or a CBOR error.
///
/// # Example
///
/// ```rust
/// use sertyp::{serialize_cbor, Integer, Item, error};
///
/// fn example(_cbor_bytes: &[u8]) -> Vec<u8> {
///     let number = Integer::i32(42);
///     let item: Item = number.into();
///     match serialize_cbor(&item) {
///         Ok(cbor_bytes) => cbor_bytes,
///         Err(e) => error!("{e}"),
///     }
/// }
/// ```
pub fn serialize_cbor(ty: &Item<'_>) -> serde_cbor::Result<Vec<u8>> {
    serde_cbor::to_vec(&ty)
}

/// Convenience macro for returning errors from plugin functions.
///
/// This macro creates a formatted error message, converts it into a Typst panic,
/// serializes it to CBOR, and returns it from the current function. When the
/// serialized panic reaches Typst, it will be raised as a runtime panic with
/// the provided message.
///
/// # Usage
///
/// This macro is supposed to be used in raw wasm plugin functions. If you use the #[typst_func] macro, consider returning a [Result] or [std::result::Result] instead.
///
/// ```rust
/// use sertyp::error;
///
/// pub fn example(cbor_bytes: &[u8]) -> Vec<u8> {
///     let item = match sertyp::deserialize_cbor(cbor_bytes) {
///         Ok(item) => item,
///         Err(e) => error!("Deserialization failed: {e}"),
///     };
///     // Further processing...
///     vec![]
/// }
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let err = format!($($arg)*);
        let s: sertyp::String = err.as_str().into();
        let p: sertyp::Panic = s.into();
        return sertyp::serialize_cbor(&p.into()).unwrap();
    }};
}
