use std::borrow::Cow;

use winnow::{
    Parser,
    ascii::{Caseless, Uint, digit0},
    combinator::{alt, trace},
    error::ParserError,
    stream::{AsChar, Compare, CompareResult, Offset, ParseSlice, Stream, StreamIsPartial},
    token::one_of,
};

#[derive(Debug, Clone)]
pub struct Bool(bool);
impl Offset for Bool {
    fn offset_from(&self, start: &Self) -> usize {
        if (start.0 as usize) > (self.0 as usize) {
            1
        } else {
            0
        }
    }
}

impl<'a> Offset<Bool> for CharStream<'a> {
    fn offset_from(&self, start: &Bool) -> usize {
        Bool(self.exhausted).offset_from(start)
    }
}

#[derive(Debug, Clone)]
pub struct CharStream<'a> {
    inner: &'a char,
    exhausted: bool,
}
impl<'a> From<&'a char> for CharStream<'a> {
    fn from(value: &'a char) -> Self {
        CharStream {
            inner: value,
            exhausted: false,
        }
    }
}
impl<'a> From<&'a Cow<'a, char>> for CharStream<'a> {
    fn from(value: &'a Cow<'a, char>) -> Self {
        CharStream {
            inner: value.as_ref(),
            exhausted: false,
        }
    }
}
impl<'a> StreamIsPartial for CharStream<'a> {
    type PartialState = bool;

    fn complete(&mut self) -> Self::PartialState {
        true
    }

    fn restore_partial(&mut self, _state: Self::PartialState) {}

    fn is_partial_supported() -> bool {
        false
    }
}
impl<'a, 'b> Compare<Caseless<&'b str>> for CharStream<'a> {
    fn compare(&self, t: Caseless<&'b str>) -> CompareResult {
        if !self.exhausted && t.0.len() == 1 && t.0.starts_with(*self.inner) {
            CompareResult::Ok(1)
        } else {
            CompareResult::Error
        }
    }
}
impl<'a> Compare<char> for CharStream<'a> {
    fn compare(&self, t: char) -> CompareResult {
        if *self.inner == t {
            CompareResult::Ok(1)
        } else {
            CompareResult::Error
        }
    }
}
impl<'a> ParseSlice<usize> for CharStream<'a> {
    fn parse_slice(&self) -> Option<usize> {
        if !self.exhausted {
            let s = self.inner.to_string();
            s.parse().ok()
        } else {
            None
        }
    }
}
impl<'a> Stream for CharStream<'a> {
    type Token = char;
    type Slice = CharStream<'a>;
    type IterOffsets = core::option::IntoIter<(usize, char)>;
    // yields either once or none
    type Checkpoint = Bool;

    fn iter_offsets(&self) -> Self::IterOffsets {
        if !self.exhausted {
            Some((0, *self.inner)).into_iter()
        } else {
            None.into_iter()
        }
    }

    fn eof_offset(&self) -> usize {
        if self.exhausted { 0 } else { 1 }
    }

    fn next_token(&mut self) -> Option<Self::Token> {
        let token = self.peek_token()?;
        self.exhausted = true;
        Some(token)
    }

    fn peek_token(&self) -> Option<Self::Token> {
        if !self.exhausted {
            Some(*self.inner)
        } else {
            None
        }
    }

    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        if !self.exhausted && predicate(*self.inner) {
            Some(0)
        } else {
            None
        }
    }

    fn offset_at(&self, tokens: usize) -> std::prelude::v1::Result<usize, winnow::error::Needed> {
        if tokens == 0 && !self.exhausted {
            Ok(0)
        } else if tokens == 1 && !self.exhausted {
            Ok(1)
        } else {
            Err(winnow::error::Needed::Unknown)
        }
    }

    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        let slice = self.peek_slice(offset);
        self.exhausted = slice.exhausted;
        slice
    }

    fn peek_slice(&self, offset: usize) -> Self::Slice {
        let mut cloned = self.clone();
        if offset == 1 || self.exhausted {
            cloned.exhausted = true;
        }
        cloned
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        Bool(self.exhausted)
    }

    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        self.exhausted = checkpoint.0;
    }

    fn trace(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "CharStream({:?}, exhausted: {})",
            self.inner, self.exhausted
        )
    }
}

pub fn dec_uint<Input, Output, Error>(input: &mut Input) -> Result<Output, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar + Clone,
    <Input as Stream>::Slice: ParseSlice<Output>,
    Output: Uint,
    Error: ParserError<Input>,
{
    trace("dec_uint", move |input: &mut Input| {
        take_dec_uint_or_exceptions(input)?
            .parse_slice()
            .ok_or_else(|| ParserError::from_input(input))
    })
    .parse_next(input)
}

fn take_dec_uint_or_exceptions<I, E: ParserError<I>>(
    input: &mut I,
) -> Result<<I as Stream>::Slice, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Token: AsChar + Clone,
{
    alt(((one_of('1'..='9'), digit0).void(), one_of('0').void()))
        .take()
        .parse_next(input)
}
