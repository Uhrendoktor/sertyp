mod charstream;
pub use charstream::{CharStream, dec_uint};

mod error;
pub use error::{Context, TypstError, error_box};

pub mod parser;

use core::panic;
use std::{
    ops::{Deref, DerefMut, Range},
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use winnow::{
    Parser, Result,
    ascii::{Caseless, multispace1},
    combinator::alt,
    error::{ContextError, ParserError},
    stream::{Compare, ContainsToken, Offset, ParseSlice, Stream, StreamIsPartial},
    token::one_of,
};

use crate::{
    Content, GroupType, LocatingSequence, PreToken, Raw, Sequence, Space, Symbol, TypedItem,
    math::{Equation, LR},
};

/// Token parser adapter with word fallback.
///
/// # Purpose
/// - Try `P` at the current position.
/// - If `P` fails, accumulate characters into `Token::Word`.
/// - Stop accumulation when `P` succeeds or input is exhausted.
///
/// # Behavior
/// - If `P` succeeds at the current position: forwards `P` result.
/// - If `P` succeeds later: returns preceding `Token::Word`.
/// - If `P` never succeeds: returns remaining input as `Token::Word`.
#[derive(Debug, Clone)]
pub struct WordFallbackParser<'a, P: Parser<&'a str, Token<'a>, ContextError> = DefaultTokenParser>
{
    parser: P,
    _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a, P: Parser<&'a str, Token<'a>, ContextError>> WordFallbackParser<'a, P> {
    /// Creates a fallback parser using `parser` as primary token parser.
    pub fn new(parser: P) -> Self {
        WordFallbackParser {
            parser,
            _marker: std::marker::PhantomData,
        }
    }
}
impl<'a, P: Parser<&'a str, Token<'a>, ContextError>> Parser<&'a str, Token<'a>, ContextError>
    for WordFallbackParser<'a, P>
{
    fn parse_next(&mut self, input: &mut &'a str) -> Result<Token<'a>, ContextError> {
        let mut i = 0;
        loop {
            match self.parser.parse_peek(&input[i..]) {
                Ok(token) if i == 0 => {
                    *input = &input[i..];
                    return self.parser.parse_next(input);
                }
                Ok(_) => {
                    let v = Ok(Token::Word(&input[..i]));
                    *input = &input[i..];
                    return v;
                }
                Err(_) if i >= input.len() => {
                    let v = Ok(Token::Word(&input[..i]));
                    *input = &input[i..];
                    return v;
                }
                Err(_) => i += 1,
            }
        }
    }
}

/// Default token parser for textual sequence fragments.
///
/// # Produces
/// - `Token::Delimiter` for operator/punctuation symbols.
/// - `Token::Number` for floating-point literals.
/// - `Token::GroupOpen` / `Token::GroupClose` for grouping symbols.
/// - `Token::Raw(&SPACE)` for one or more whitespace characters.
#[derive(Debug, Clone)]
pub struct DefaultTokenParser;
impl<
    'a,
    I: Stream<Token = char> + StreamIsPartial + Compare<Caseless<&'static str>> + Compare<char>,
    E: ParserError<I>,
> Parser<I, Token<'a>, E> for DefaultTokenParser
where
    <I as Stream>::Slice: ParseSlice<usize>,
    <I as Stream>::IterOffsets: Clone,
{
    fn parse_next(&mut self, input: &mut I) -> Result<Token<'a>, E> {
        alt((
            one_of([
                '+', '-', '−', '*', '/', '%', '=', '<', '>', '!', '?', '&', '|', '^', '~',
            ])
            .map(Token::Delimiter),
            one_of([
                '±', '∓', '×', '÷', '∗', '∙', '≠', '≈', '≃', '≅', '≤', '≥', '∧', '∨', '¬',
            ])
            .map(Token::Delimiter),
            dec_uint.map(Token::Number),
            one_of([';', ':', ',', '.', '…', '·', '•', '@', '#', '$']).map(Token::Delimiter),
            one_of([
                '(', '[', '{', '<', '«', '‹', '“', '‘', '„', '‚', '⟨', '⟪', '⟮', '〈', '⌈', '⌊',
                '⦇', '⦃', '⦅',
            ])
            .map(Token::GroupOpen),
            one_of([
                ')', ']', '}', '>', '»', '›', '”', '’', '‟', '‛', '⟩', '⟫', '⟯', '〉', '⌉', '⌋',
                '⦈', '⦄', '⦆',
            ])
            .map(Token::GroupClose),
            one_of(['|', '¦', '‖']).map(Token::Delimiter),
            one_of(['\\', '`', '\'', '"', '_']).map(Token::Delimiter),
            multispace1.map(|_| Token::Raw(&SPACE)),
        ))
        .parse_next(input)
    }
}

/// Newtype wrapper for `usize` used as stream checkpoints.
/// Enables implementing winnow's `Offset` trait for checkpoint math.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Usize(usize);
impl Deref for Usize {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Usize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<usize> for Usize {
    fn from(value: usize) -> Self {
        Usize(value)
    }
}
impl Offset for Usize {
    fn offset_from(&self, start: &Self) -> usize {
        self.0 - start.0
    }
}

impl<'a> Offset<Usize> for LocatingSequence<'a> {
    /// Offset is calculated as the number of tokens
    fn offset_from(&self, start: &Usize) -> usize {
        (*self.pos).fetch_sub(**start, Ordering::Relaxed)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Locatable<T> {
    pub inner: T,
    /// Global offset of token start.
    pub offset: usize,
    /// Token width in stream units. Only >1 for subtokens of [Text].
    pub len: usize,
}
impl<T> Locatable<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Locatable<U> {
        Locatable {
            inner: f(self.inner),
            offset: self.offset,
            len: self.len,
        }
    }
}
impl<T> Deref for Locatable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T> DerefMut for Locatable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Token plus source-location metadata used by `LocatingSequence`.
pub type LocatingToken<'a> = Locatable<Token<'a>>;

#[derive(Debug, Clone)]
/// Token kind emitted by text/content sequence tokenization.
pub enum Token<'a> {
    /// Fallback token for all [Content] that does not have a direct textual representation. Treated as a single token with length=1.
    Raw(&'a Content<'a>),
    /// Delimiter or operator symbol.
    Delimiter(char),
    /// numeric literal.
    Number(usize),
    /// Fallback token for unrecognized sequences of characters. Contains the raw string slice.
    Word(&'a str),
    /// Opening grouping symbol.
    GroupOpen(char),
    /// Closing grouping symbol.
    GroupClose(char),
    /// Start marker for embedded math sequence.
    MathOpen,
    /// End marker for embedded math sequence.
    MathClose,
    /// Start marker for nested sequence.
    SequenceOpen,
    /// End marker for nested sequence.
    SequenceClose,
}
impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Raw(l0), Self::Raw(r0)) => std::ptr::eq(*l0, *r0),
            (Self::Delimiter(l0), Self::Delimiter(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Word(l0), Self::Word(r0)) => l0 == r0,
            (Self::GroupOpen(l0), Self::GroupOpen(r0)) => l0 == r0,
            (Self::GroupClose(l0), Self::GroupClose(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'a> Parser<LocatingSequence<'a>, LocatingToken<'a>, TypstError<'a>> for Token<'a> {
    fn parse_next(
        &mut self,
        input: &mut LocatingSequence<'a>,
    ) -> Result<LocatingToken<'a>, TypstError<'a>> {
        if let Some(token) = input.next_token() {
            if self == &token.inner {
                Ok(token)
            } else {
                Err(TypstError::from_token(&token)
                    .context(Context::Label("token".into()))
                    .context(Context::Expected(format!("{self:#?}").into()))
                    .context(Context::Found(format!("{token:#?}").into())))
            }
        } else {
            Err(TypstError::from_input(input)
                .context(Context::Label("token".into()))
                .context(Context::Expected(format!("{self:#?}").into()))
                .context(Context::Found("EOF".into())))
        }
    }
}
impl<'a, const N: usize> ContainsToken<Token<'a>> for [Token<'a>; N] {
    fn contains_token(&self, token: Token<'a>) -> bool {
        self.contains(&token)
    }
}
impl<'a, const N: usize> ContainsToken<LocatingToken<'a>> for [Token<'a>; N] {
    fn contains_token(&self, token: LocatingToken<'a>) -> bool {
        self.contains_token(token.inner)
    }
}

impl<'a> Iterator for LocatingSequence<'a> {
    type Item = (usize, LocatingToken<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        // next_token increments pos
        self.next_token().map(|token| (self.pos() - 1, token))
    }
}

static SPACE: Content = Content::Space(Space);
impl<'a> LocatingSequence<'a> {
    /// Creates an independent cursor over the same token storage.
    ///
    /// # Notes
    /// - Copies current position into a new atomic cursor.
    /// - Advancing the clone does not modify the original cursor.
    pub fn lookahead(&self) -> LocatingSequence<'a> {
        // creates new position tracker for lookahead, to not affect the original one.
        LocatingSequence {
            tokens: self.tokens.clone(),
            pos: Rc::new(self.pos.load(Ordering::Relaxed).into()),
            len: self.len,
            offset: self.offset,
        }
    }

    /// Returns absolute stream position.
    pub fn global_pos(&self) -> usize {
        self.pos.load(Ordering::Relaxed)
    }

    /// Returns position relative to this view's local offset.
    pub fn pos(&self) -> usize {
        self.pos.fetch_sub(self.offset, Ordering::Relaxed)
    }

    fn peek_len(&self) -> Option<LocatingToken<'a>> {
        if self.pos() >= self.len {
            return None;
        }
        fn parser<'a>(text: &'a str, pos: &usize, start: &usize) -> Option<LocatingToken<'a>> {
            WordFallbackParser::new(DefaultTokenParser)
                .with_taken()
                .parse_next(&mut &text[pos - *start..])
                .map(|(token, v)| LocatingToken {
                    inner: token,
                    offset: *pos,
                    len: v.len(),
                })
                .ok()
        }
        match self.tokens.get(&self.pos())? {
            PreToken::Token {
                token: Content::Text(text),
                start,
                ..
            } => parser(text.as_string(), &self.pos(), start),
            PreToken::Token {
                token: Content::Raw(Raw { text, .. }),
                start,
                ..
            } => parser(&**text, &self.pos(), start),
            PreToken::Token {
                token: Content::Symbol(Symbol(s)),
                ..
            } => {
                let r: Result<_, ContextError> = DefaultTokenParser
                    .parse_next(&mut CharStream::from(s))
                    .map(|token| LocatingToken {
                        inner: token,
                        offset: self.pos(),
                        len: 1,
                    });
                r.ok()
            }
            PreToken::Token { token, .. } => Some(LocatingToken {
                inner: Token::Raw(token),
                offset: self.pos(),
                len: 1,
            }),
            PreToken::Open(GroupType::Math) => Some(LocatingToken {
                inner: Token::MathOpen,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::Close(GroupType::Math) => Some(LocatingToken {
                inner: Token::MathClose,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::Open(GroupType::Sequence) => Some(LocatingToken {
                inner: Token::SequenceOpen,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::Close(GroupType::Sequence) => Some(LocatingToken {
                inner: Token::SequenceClose,
                offset: self.pos(),
                len: 1,
            }),
        }
    }
}

impl<'a> Stream for LocatingSequence<'a> {
    type Token = LocatingToken<'a>;
    type Slice = Self;
    type IterOffsets = LocatingSequence<'a>;
    type Checkpoint = Usize;

    fn iter_offsets(&self) -> Self::IterOffsets {
        self.clone()
    }

    fn eof_offset(&self) -> usize {
        self.len.saturating_sub(self.pos())
    }

    fn next_token(&mut self) -> Option<Self::Token> {
        if let Some(token) = self.peek_len() {
            let _ = self
                .pos
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |pos| {
                    Some(pos + token.len)
                }); // Move to the next token
            return Some(token);
        }
        None
    }

    fn peek_token(&self) -> Option<Self::Token> {
        self.peek_len()
    }

    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        let lookahead = self.lookahead();
        for (offset, token) in lookahead.iter_offsets() {
            if predicate(token) {
                return Some(offset);
            }
        }
        None
    }

    fn offset_at(&self, tokens: usize) -> Result<usize, winnow::error::Needed> {
        let mut lookahead = self.lookahead();
        lookahead.pos.store(lookahead.offset, Ordering::Relaxed);
        for _ in 0..tokens {
            if lookahead.next_token().is_none() {
                break;
            }
        }
        Ok(lookahead.pos.load(Ordering::Relaxed))
    }

    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        let slice = self.peek_slice(offset);
        self.pos.store(offset, Ordering::Relaxed);
        slice
    }

    fn peek_slice(&self, offset: usize) -> Self::Slice {
        match offset {
            _ if offset > self.len => {
                panic!("Offset out of bounds: offset {}, len {}", offset, self.len)
            }
            _ => (),
        };
        LocatingSequence {
            tokens: self.tokens.clone(),
            pos: Rc::new(offset.into()),
            len: self.len - offset,
            offset: self.offset + offset,
        }
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.pos.load(Ordering::Relaxed).into()
    }

    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        self.pos.store(**checkpoint, Ordering::Relaxed);
    }

    fn trace(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Token {} at position {}",
            self.peek_token()
                .map_or("EOF".to_string(), |t| format!("{:?}", t)),
            self.pos()
        )
    }
}

impl<'a> StreamIsPartial for LocatingSequence<'a> {
    type PartialState = bool;

    fn complete(&mut self) -> Self::PartialState {
        true
    }

    fn restore_partial(&mut self, _state: Self::PartialState) {}

    fn is_partial_supported() -> bool {
        false
    }
}
