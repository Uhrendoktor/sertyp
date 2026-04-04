use sertyp::{Content, LocatingSequence, LocatingToken, Text, Token, TypstError, typst_func};
use wasm_minimal_protocol::*;
use winnow::{
    error::{ParserError, StrContext, StrContextValue},
    stream::Stream,
};

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

/// Does a full cycle of deserialization and serialization for test purposes.
#[typst_func]
pub fn cycle<'a>(value: sertyp::Item<'a>) -> sertyp::Item<'a> {
    value
}

/// Dummy function that does not expect a sertyp::Panic as input and therefore cascades the error if it receives one.
#[typst_func]
pub fn not_expecting_error<'a>(value: sertyp::Content<'a>) -> sertyp::Content<'a> {
    value
}
