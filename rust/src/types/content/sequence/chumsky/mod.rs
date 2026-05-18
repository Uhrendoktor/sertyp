use chumsky::{
    input::{ExactSizeInput, Input, SliceInput, ValueInput},
    span::SimpleSpan,
};

use crate::{Content, GroupType, LocatingSequence, PreToken};

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Raw(&'a Content<'a>),
    Char(char),
    Open(&'a GroupType),
    Close(&'a GroupType),
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Raw(c1), Token::Raw(c2)) => std::ptr::eq(*c1, *c2),
            (Token::Char(c1), Token::Char(c2)) => c1 == c2,
            (Token::Open(g1), Token::Open(g2)) => std::ptr::eq(*g1, *g2),
            (Token::Close(g1), Token::Close(g2)) => std::ptr::eq(*g1, *g2),
            _ => false,
        }
    }
}

impl<'a> Input<'a> for &'a LocatingSequence<'a> {
    type Span = SimpleSpan<usize>;
    type Token = Token<'a>;
    type MaybeToken = Token<'a>;
    type Cursor = usize;
    type Cache = Self;

    fn begin(self) -> (Self::Cursor, Self::Cache) {
        (0, self)
    }

    fn cursor_location(cursor: &Self::Cursor) -> usize {
        *cursor
    }

    unsafe fn next_maybe(
        this: &mut Self::Cache,
        cursor: &mut Self::Cursor,
    ) -> Option<Self::MaybeToken> {
        let token = this.tokens.get(cursor)?;
        let token = match token {
            PreToken::Token {
                token: Content::Text(text),
                start,
                ..
            } => {
                let c = text
                    .as_string()
                    .get(*cursor - start..)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
                *cursor += c.len_utf8();
                Token::Char(c)
            }
            PreToken::Token { token, .. } => {
                *cursor += 1;
                Token::Raw(token)
            }
            PreToken::Open(group_type) => Token::Open(group_type),
            PreToken::Close(group_type) => Token::Close(group_type),
        };
        Some(token)
    }

    unsafe fn span(_this: &mut Self::Cache, range: std::ops::Range<&Self::Cursor>) -> Self::Span {
        SimpleSpan::from(*range.start..*range.end)
    }
}

impl<'a> ValueInput<'a> for &'a LocatingSequence<'a> {
    unsafe fn next(this: &mut Self::Cache, cursor: &mut Self::Cursor) -> Option<Self::Token> {
        unsafe { Self::next_maybe(this, cursor) }
    }
}

impl<'a> ExactSizeInput<'a> for &'a LocatingSequence<'a> {
    unsafe fn span_from(
        this: &mut Self::Cache,
        range: std::ops::RangeFrom<&Self::Cursor>,
    ) -> Self::Span {
        SimpleSpan::from(*range.start..this.tokens.len())
    }
}

impl<'a> SliceInput<'a> for &'a LocatingSequence<'a> {
    type Slice = Vec<Token<'a>>;

    fn full_slice(this: &mut Self::Cache) -> Self::Slice {
        unsafe { Self::slice(this, &0..&this.tokens.len()) }
    }

    unsafe fn slice(this: &mut Self::Cache, range: std::ops::Range<&Self::Cursor>) -> Self::Slice {
        let mut cursor = *range.start;
        let mut slice = Vec::new();
        while cursor < *range.end {
            if let Some(next) = unsafe { Self::next(this, &mut cursor) } {
                slice.push(next);
            }
        }
        slice
    }

    unsafe fn slice_from(
        this: &mut Self::Cache,
        from: std::ops::RangeFrom<&Self::Cursor>,
    ) -> Self::Slice {
        unsafe { Self::slice(this, from.start..&this.tokens.len()) }
    }
}
