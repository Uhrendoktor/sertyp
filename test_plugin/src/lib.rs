use sertyp::{LocatingSequence, Token, typst_func};
use wasm_minimal_protocol::*;
use winnow::{
    Parser,
    combinator::alt,
    error::{AddContext, ContextError, ParserError},
    stream::Stream,
    token::take_while,
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

#[typst_func]
pub fn test_sequence<'a>(
    seq: sertyp::TypedContent<sertyp::Sequence<'a>>,
) -> Result<sertyp::TypedArray<sertyp::String<'a>>, sertyp::String<'a>> {
    let mut locating_seq: LocatingSequence = (&seq.0).into();
    let mut v: Vec<sertyp::String<'a>> = vec![];
    while let Some(token) = locating_seq.next_token() {
        v.push(format!("{:#?}", token).into());
    }
    Ok(v.into())
}
