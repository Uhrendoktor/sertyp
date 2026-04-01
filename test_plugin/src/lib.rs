use sertyp::{LocatingSequence, LocatingToken, Text, Token, TypstError, typst_func};
use wasm_minimal_protocol::*;
use winnow::{
    Parser,
    error::{StrContext, StrContextValue},
    stream::{ContainsToken, Stream},
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
    let mut locating_seq: LocatingSequence = (&*seq).into();
    let mut v: Vec<sertyp::String<'a>> = vec![];
    while let Some(token) = locating_seq.next_token() {
        v.push(format!("{:#?}", token).into());
    }
    Ok(v.into())
}

fn until_parser(l: &mut LocatingSequence) -> Result<(), TypstError> {
    struct TakeWhile {
        s: std::cell::RefCell<String>,
    }
    impl ContainsToken<LocatingToken<'_>> for &TakeWhile {
        fn contains_token(&self, token: LocatingToken<'_>) -> bool {
            match token {
                LocatingToken {
                    token: Token::Delimiter(t),
                    ..
                } => {
                    self.s.borrow_mut().push(t);
                    false
                }
                _ => true,
            }
        }
    }
    let t = TakeWhile {
        s: Default::default(),
    };
    take_while(10.., &t)
        .context(StrContext::Label("character"))
        .context(StrContext::Expected(StrContextValue::Description(
            Box::leak(Box::new(format!("delimiter {:?}", t.s.borrow()))),
        )))
        .parse_next(l)
        .map(|_| ())
}

#[typst_func(&)]
pub fn test_sequence2<'a>(
    seq: &'a sertyp::TypedContent<sertyp::Sequence<'a>>,
) -> Result<sertyp::Content<'a>, sertyp::String<'a>> {
    let mut locating_seq: LocatingSequence = (&**seq).into();
    // loop {
    let token: Result<(), _> = until_parser.parse_next(&mut locating_seq);
    match token {
        Ok(_) => {}
        Err(e) => {
            return Ok(e.render(&locating_seq).into());
        } //Ok(locating_seq.mark_error(&e).into()),
    };
    Ok(Text::from_string(format!("End {:?}", locating_seq)).into())
    // }
}
