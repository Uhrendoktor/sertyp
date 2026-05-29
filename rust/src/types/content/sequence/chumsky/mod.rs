use std::fmt::Debug;

use chumsky::{
    DefaultExpected,
    error::Error,
    input::{ExactSizeInput, Input, SliceInput, ValueInput},
    label::LabelError,
    span::SimpleSpan,
    util::MaybeRef,
};

use crate::{
    Content, GroupType, LocatingSequence, PreToken,
    error::{Context, TypstError},
};

pub mod parser;

/// Token representation exposed to the `chumsky` parser.
///
/// - `Raw` wraps a reference to a `Content` value (non-text content).
/// - `Char` represents a single character taken from `Text` content.
/// - `Open`/`Close` mirror `GroupType` boundaries used for grouped constructs.
#[derive(Debug, Clone)]
pub enum Token<'this, 'data> {
    Raw(&'this Content<'data>),
    Char(char),
    Open(GroupType),
    Close(GroupType),
}

impl PartialEq for Token<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Raw(c1), Token::Raw(c2)) => std::ptr::eq(*c1, *c2),
            (Token::Char(c1), Token::Char(c2)) => c1 == c2,
            (Token::Open(g1), Token::Open(g2)) => {
                std::mem::discriminant(g1) == std::mem::discriminant(g2)
            }
            (Token::Close(g1), Token::Close(g2)) => {
                std::mem::discriminant(g1) == std::mem::discriminant(g2)
            }
            _ => false,
        }
    }
}

impl<'this, 'data> Input<'this> for &'this LocatingSequence<'this, 'data> {
    type Span = SimpleSpan<usize>;
    type Token = Token<'this, 'data>;
    type MaybeToken = Token<'this, 'data>;
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
            PreToken::Token {
                token: Content::Symbol(s),
                ..
            } => {
                let c: char = **s;
                *cursor += c.len_utf8();
                Token::Char(c)
            }
            PreToken::Token { token, .. } => {
                *cursor += 1;
                Token::Raw(token)
            }
            PreToken::Open(group_type) => {
                *cursor += 1;
                Token::Open(group_type.clone())
            }
            PreToken::Close(group_type) => {
                *cursor += 1;
                Token::Close(group_type.clone())
            }
        };
        Some(token)
    }

    unsafe fn span(_this: &mut Self::Cache, range: std::ops::Range<&Self::Cursor>) -> Self::Span {
        SimpleSpan::from(*range.start..*range.end)
    }
}

impl<'this, 'data> Input<'this> for LocatingSequence<'this, 'data> {
    type Span = SimpleSpan<usize>;
    type Token = Token<'this, 'data>;
    type MaybeToken = Token<'this, 'data>;
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
            PreToken::Open(group_type) => Token::Open(group_type.clone()),
            PreToken::Close(group_type) => Token::Close(group_type.clone()),
        };
        Some(token)
    }

    unsafe fn span(_this: &mut Self::Cache, range: std::ops::Range<&Self::Cursor>) -> Self::Span {
        SimpleSpan::from(*range.start..*range.end)
    }
}

impl<'this, 'data> ValueInput<'this> for &'this LocatingSequence<'this, 'data> {
    unsafe fn next(this: &mut Self::Cache, cursor: &mut Self::Cursor) -> Option<Self::Token> {
        unsafe { Self::next_maybe(this, cursor) }
    }
}

impl<'this, 'data> ValueInput<'this> for LocatingSequence<'this, 'data> {
    unsafe fn next(this: &mut Self::Cache, cursor: &mut Self::Cursor) -> Option<Self::Token> {
        unsafe { Self::next_maybe(this, cursor) }
    }
}

impl<'this, 'data> ExactSizeInput<'this> for &'this LocatingSequence<'this, 'data> {
    unsafe fn span_from(
        this: &mut Self::Cache,
        range: std::ops::RangeFrom<&Self::Cursor>,
    ) -> Self::Span {
        SimpleSpan::from(*range.start..this.tokens.len())
    }
}

impl<'this, 'data> ExactSizeInput<'this> for LocatingSequence<'this, 'data> {
    unsafe fn span_from(
        this: &mut Self::Cache,
        range: std::ops::RangeFrom<&Self::Cursor>,
    ) -> Self::Span {
        SimpleSpan::from(*range.start..this.tokens.len())
    }
}

impl<'this, 'data> SliceInput<'this> for &'this LocatingSequence<'this, 'data> {
    type Slice = Vec<Token<'this, 'data>>;

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

impl<'this, 'data> SliceInput<'this> for LocatingSequence<'this, 'data> {
    type Slice = Vec<Token<'this, 'data>>;

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

/// Helper trait alias used when implementing parsers over a `LocatingSequence`.
/// It bundles the necessary `chumsky` input traits so implementations can be
/// generic over either `&LocatingSequence` or `LocatingSequence`.
pub trait LocatingSequenceLike<'this, 'data: 'this>:
    Input<
        'this,
        Span = SimpleSpan<usize>,
        Token = Token<'this, 'data>,
        MaybeToken = Token<'this, 'data>,
        Cursor = usize,
    > + ValueInput<'this>
    + ExactSizeInput<'this>
    + SliceInput<'this>
{
}
impl<'this, 'data> LocatingSequenceLike<'this, 'data> for &'this LocatingSequence<'this, 'data> {}
impl<'this, 'data> LocatingSequenceLike<'this, 'data> for LocatingSequence<'this, 'data> {}

impl<'this, 'data: 'this, I: Input<'this>> Error<'this, I> for TypstError<'data, I::Span> where
    I::Token: Debug
{
}

impl<'this, 'data: 'this, I: Input<'this>> LabelError<'this, I, crate::String<'data>>
    for TypstError<'data, I::Span>
where
    I::Token: Debug,
{
    fn label_with(&mut self, label: crate::String<'data>) {
        self.context_mut(Context::Label(label));
    }

    fn expected_found<E: IntoIterator<Item = crate::String<'data>>>(
        expected: E,
        found: Option<MaybeRef<'this, <I as Input<'this>>::Token>>,
        span: <I as Input<'this>>::Span,
    ) -> Self {
        let mut error = TypstError::spanned(span);
        for e in expected {
            error.context_mut(Context::Expected(e));
        }
        if let Some(found) = found {
            error.context_mut(Context::Found(format!("{:#?}", found).into()));
        }
        error
    }
}

impl<'this, 'data: 'this, I: Input<'this>> LabelError<'this, I, DefaultExpected<'this, I::Token>>
    for TypstError<'data, I::Span>
where
    I::Token: Debug,
{
    fn label_with(&mut self, label: DefaultExpected<'this, I::Token>) {
        self.context_mut(Context::Label(format!("{:?}", label).into()));
    }

    fn expected_found<E: IntoIterator<Item = DefaultExpected<'this, I::Token>>>(
        expected: E,
        found: Option<MaybeRef<'this, I::Token>>,
        span: I::Span,
    ) -> Self {
        let mut error = TypstError::spanned(span);
        for e in expected {
            error.context_mut(Context::Expected(match e {
                DefaultExpected::Token(t) => format!("{:?}", t).into(),
                t => format!("{:?}", t).into(),
            }));
        }
        if let Some(found) = found {
            error.context_mut(Context::Found(format!("{:#?}", found).into()));
        }
        error
    }
}
