use core::panic;
use std::{
    ops::{Deref, DerefMut, Range},
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use winnow::{
    Parser, Result,
    ascii::{float, multispace1},
    combinator::alt,
    error::{AddContext, ContextError, ParserError, StrContext, StrContextValue},
    stream::{Offset, Stream, StreamIsPartial},
    token::one_of,
};

use crate::{
    Box, Color, Content, Length, Or, Panic, Place, RBox, Raw, Sequence, Space, Stroke, Text,
    TypedItem, Underline, math::Equation, types::generic::FillColor,
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
impl<'a> Default for WordFallbackParser<'a> {
    fn default() -> Self {
        WordFallbackParser::new(DefaultTokenParser)
    }
}

/// Default token parser for textual sequence fragments.
///
/// # Produces
/// - `Token::Delimiter` for operator/punctuation symbols.
/// - `Token::Number` for floating-point literals.
/// - `Token::GroupOpen` / `Token::GroupClose` for grouping symbols.
/// - `Token::Raw(&SPACE)` for one or more whitespace characters.
pub struct DefaultTokenParser;
impl<'a, E: ParserError<&'a str>> Parser<&'a str, Token<'a>, E> for DefaultTokenParser {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<Token<'a>, E> {
        alt((
            one_of([
                '+', '-', '*', '/', '%', '=', '<', '>', '!', '?', '&', '|', '^', '~',
            ])
            .map(Token::Delimiter),
            one_of([
                '±', '∓', '×', '÷', '∗', '∙', '≠', '≈', '≃', '≅', '≤', '≥', '∧', '∨', '¬',
            ])
            .map(Token::Delimiter),
            float.map(Token::Number),
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

#[derive(Debug, Clone)]
enum PreToken<'a> {
    Token {
        token: &'a Content<'a>,
        start: usize,
        len: usize,
    },
    MathOpen,
    MathClose,
    SequenceOpen,
    SequenceClose,
}
/// Impls pointer compare for PreTokens, as equals is only used when inserting Tokens into the rangemap.
impl<'a> PartialEq for PreToken<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PreToken::Token { token: t1, .. }, PreToken::Token { token: t2, .. }) => {
                std::ptr::eq(t1, t2)
            }
            (PreToken::MathOpen, PreToken::MathOpen) => true,
            (PreToken::MathClose, PreToken::MathClose) => true,
            (PreToken::SequenceOpen, PreToken::SequenceOpen) => true,
            (PreToken::SequenceClose, PreToken::SequenceClose) => true,
            _ => false,
        }
    }
}
impl<'a> Eq for PreToken<'a> {}

#[derive(Debug, Clone)]
struct PreTokenMap<'a> {
    /// indexing is done by inbetween-token offsets. Each character in a string has a offset
    tokens: rangemap::RangeMap<usize, PreToken<'a>>,
}
impl<'a> PreTokenMap<'a> {
    fn new() -> Self {
        PreTokenMap {
            tokens: rangemap::RangeMap::new(),
        }
    }
    fn insert_token(&mut self, content: &'a Content<'a>, range: Range<usize>) {
        self.tokens.insert(
            range.clone(),
            PreToken::Token {
                token: content,
                start: range.start,
                len: range.end - range.start,
            },
        );
    }
    fn get(&self, offset: &usize) -> Option<&PreToken<'a>> {
        self.tokens.get(offset)
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

#[derive(Debug, Clone)]
/// Stream adapter over `Sequence` with stable position tracking.
///
/// # Purpose
/// - Expose sequence content as a winnow `Stream`.
/// - Preserve token offsets for precise parser errors.
/// - Support lookahead and slicing without losing global positions.
pub struct LocatingSequence<'a> {
    /// unparsed tokens represented by [Content]
    tokens: PreTokenMap<'a>,
    /// shared reference of position.
    /// [Rc<AtomicUsize>] Neccessary for interior mutability and correct error reporting,
    /// as `iter_offsets` only provides a `&` reference.
    pos: Rc<AtomicUsize>,
    /// cummulative length or [PreToken]s. [Text] contribute their respective number of [char]s, everything else has length=1.
    len: usize,
    /// starting position in relation to indices used in [PreTokenMap]. Can be unequal to 0 in cases where sub-sequences are created (lookahead, etc.).
    offset: usize,
}
pub struct TypstError {
    /// Global token/character offset where parsing failed.
    pub offset: usize,
    /// Span length of the failing token/segment.
    pub len: usize,
    /// Underlying winnow context error.
    pub inner: ContextError,
}
impl TypstError {
    /// Appends a context entry to the underlying error and returns `self`.
    pub fn context(mut self, context: StrContext) -> Self {
        self.inner.push(context);
        self
    }

    /// Creates an error positioned at `token`.
    pub fn from_token(token: &LocatingToken<'_>) -> Self {
        TypstError {
            offset: token.offset,
            len: token.len,
            inner: ContextError::new(),
        }
    }
}
impl ParserError<LocatingSequence<'_>> for TypstError {
    type Inner = Self;

    fn from_input(input: &LocatingSequence<'_>) -> Self {
        TypstError {
            offset: input.global_pos(),
            len: 1,
            inner: ContextError::new(),
        }
    }

    fn into_inner(self) -> Result<Self::Inner, Self> {
        Ok(self)
    }
}
impl AddContext<LocatingSequence<'_>, StrContext> for TypstError {
    #[inline]
    fn add_context(
        mut self,
        _input: &LocatingSequence<'_>,
        _token_start: &<LocatingSequence<'_> as Stream>::Checkpoint,
        context: StrContext,
    ) -> Self {
        self.inner.push(context);
        self
    }
}

/// Builds an absolute positioned floating Typst error. Similar to a normal [Panic].
pub fn error_box<'a>(error: &TypstError) -> Content<'a> {
    let expression: Option<&&'static str> = error.inner.context().find_map(|c| match c {
        StrContext::Label(c) => Some(c),
        _ => None,
    });
    let expected = error
        .inner
        .context()
        .filter_map(|c| match c {
            StrContext::Expected(StrContextValue::Description(d)) => Some(*d),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(", ");
    Content::Box(
        Box {
            body: Some(RBox(
                TypedItem::new(Content::Place(Place {
                    body: Some(
                        TypedItem::new(Content::Panic(Panic {
                            ty: format!("invalid {}", *expression.unwrap_or(&"<unknown>")).into(),
                            msg: format!("expected {}", expected).into(),
                        }))
                        .into(),
                    ),
                    dy: Some(TypedItem::new(Length::pt(2.0).into())),
                    dx: Some(TypedItem::new(Length::pt(-20.0).into())),
                    ..Default::default()
                }))
                .into(),
            )),
            ..Default::default()
        }
        .into(),
    )
}
impl TypstError {
    /// Reconstructs a `Sequence` and injects visual error annotations.
    ///
    /// # Behavior
    /// - Preserves original structure (`sequence`, `math`, raw/content tokens).
    /// - Underlines the error span in red.
    /// - Inserts [`error_box`] after the highlighted span.
    pub fn render<'a>(&self, sequence: &LocatingSequence<'a>) -> Sequence<'a> {
        let mut seq: Sequence = Sequence::new();
        let mut stack = vec![&mut seq as *mut Sequence<'a>];

        unsafe fn cur<'a: 'b, 'b>(stack: &Vec<*mut Sequence<'a>>) -> &'b mut Sequence<'a> {
            unsafe { &mut **stack.last().unwrap() }
        }

        fn push<'a>(stack: &Vec<*mut Sequence<'a>>, content: Content<'a>) {
            unsafe { cur(stack) }.push(content);
        }

        fn push_group<'a>(
            stack: &mut Vec<*mut Sequence<'a>>,
            content: Content<'a>,
            f: impl for<'b> Fn(&'b mut Content<'a>) -> &'b mut Sequence<'a>,
        ) {
            push(stack, content);
            let seq_ptr = f(unsafe { cur(stack) }.last_mut().unwrap()) as *mut Sequence<'a>;
            stack.push(seq_ptr);
        }

        let mut offset = 0;
        while let Some(token) = sequence.tokens.get(&offset) {
            match token {
                PreToken::Token {
                    token, start, len, ..
                } => {
                    if (*start <= self.offset) && (self.offset < start + len) {
                        let (pre, error, post): (Option<Content<'_>>, _, Option<Content<'_>>) =
                            match token {
                                Content::Text(text) => {
                                    let b1 = self.offset.saturating_sub(*start);
                                    let b2 = b1.saturating_add(self.len);
                                    (
                                        Some(Text::from_string(&text.as_string()[..b1]).into()),
                                        Text::from_string(&text.as_string()[b1..b2]).into(),
                                        Some(Text::from_string(&text.as_string()[b2..]).into()),
                                    )
                                }
                                token => (None, (*token).clone(), None),
                            };

                        if let Some(pre) = pre {
                            push(&stack, pre);
                        }
                        push(
                            &stack,
                            Underline {
                                stroke: Some(Or::Right(Stroke {
                                    paint: Or::Right(FillColor::Color(
                                        Color::rgba_hex("#FF0000").unwrap(),
                                    )),
                                    ..Default::default()
                                })),
                                evade: Some(TypedItem(false.into())),
                                extent: Some(TypedItem(Length::pt(1.5))),
                                body: Some(RBox::new(error.into())),
                                ..Default::default()
                            }
                            .into(),
                        );
                        push(&stack, error_box(self));
                        if let Some(post) = post {
                            push(&stack, post);
                        }
                    } else {
                        push(&stack, (*token).clone());
                    }
                    offset += len;
                }
                PreToken::MathOpen => {
                    push_group(
                        &mut stack,
                        Content::MathEquation(Equation::new(Sequence::new().into())),
                        |content| match content {
                            Content::MathEquation(Equation {
                                body: TypedItem(body),
                                ..
                            }) => match &mut **body {
                                Content::Sequence(seq) => seq,
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        },
                    );
                    offset += 1;
                }
                PreToken::SequenceOpen => {
                    push_group(
                        &mut stack,
                        Sequence::new().into(),
                        |content| match content {
                            Content::Sequence(seq) => seq,
                            _ => unreachable!(),
                        },
                    );
                    offset += 1;
                }
                PreToken::MathClose | PreToken::SequenceClose => {
                    stack.pop();
                    offset += 1;
                }
            };
        }

        seq
    }
}

impl<'a> From<&'a Sequence<'a>> for LocatingSequence<'a> {
    fn from(seq: &'a Sequence<'a>) -> Self {
        let mut tokens = PreTokenMap::new();

        fn insert_content<'a>(
            pos: &mut usize,
            content: &'a Content<'a>,
            tokens: &mut PreTokenMap<'a>,
        ) {
            match content {
                Content::Text(text) => {
                    let len: usize = text.as_string().len();
                    tokens.insert_token(content, *pos..*pos + len);
                    *pos += len;
                }
                Content::Raw(Raw {
                    text: TypedItem(text),
                    ..
                }) => {
                    let len: usize = text.len();
                    tokens.insert_token(content, *pos..*pos + len);
                    *pos += len;
                }
                Content::Sequence(seq) => {
                    tokens.tokens.insert(*pos..*pos + 1, PreToken::SequenceOpen);
                    *pos += 1;
                    insert_sequence(pos, seq, tokens);
                    tokens
                        .tokens
                        .insert(*pos..*pos + 1, PreToken::SequenceClose);
                    *pos += 1;
                }
                Content::MathEquation(Equation { body, .. }) => {
                    tokens.tokens.insert(*pos..*pos + 1, PreToken::MathOpen);
                    *pos += 1;
                    insert_content(pos, body, tokens);
                    tokens.tokens.insert(*pos..*pos + 1, PreToken::MathClose);
                    *pos += 1;
                }
                content => {
                    tokens.insert_token(content, *pos..*pos + 1);
                    *pos += 1;
                }
            }
        }
        fn insert_sequence<'a>(
            pos: &mut usize,
            seq: &'a Sequence<'a>,
            tokens: &mut PreTokenMap<'a>,
        ) {
            for content in seq.as_slice().iter() {
                insert_content(pos, content, tokens);
            }
        }
        let mut pos = 0;
        insert_sequence(&mut pos, seq, &mut tokens);
        LocatingSequence {
            tokens,
            pos: Rc::new(0.into()),
            len: pos,
            offset: 0,
        }
    }
}

impl<'a> Offset<Usize> for LocatingSequence<'a> {
    /// Offset is calculated as the number of tokens
    fn offset_from(&self, start: &Usize) -> usize {
        (*self.pos).fetch_sub(**start, Ordering::Relaxed)
    }
}

#[derive(Debug, Clone)]
/// Token plus source-location metadata used by `LocatingSequence`.
pub struct LocatingToken<'a> {
    /// Parsed token value.
    pub token: Token<'a>,
    /// Global offset of token start.
    pub offset: usize,
    /// Token width in stream units. Only >1 for subtokens of [Text].
    pub len: usize,
}
#[derive(Debug, Clone)]
/// Token kind emitted by text/content sequence tokenization.
pub enum Token<'a> {
    /// Fallback token for all [Content] that does not have a direct textual representation. Treated as a single token with length=1.
    Raw(&'a Content<'a>),
    /// Delimiter or operator symbol.
    Delimiter(char),
    /// Floating-point numeric literal.
    Number(f32),
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
impl<'a> Parser<LocatingSequence<'a>, Self, TypstError> for Token<'a> {
    fn parse_next(&mut self, input: &mut LocatingSequence<'a>) -> Result<Self, TypstError> {
        match (self, input.next_token()) {
            (
                Token::Delimiter(c1),
                Some(
                    token @ LocatingToken {
                        token: Token::Delimiter(c2),
                        ..
                    },
                ),
            ) if *c1 == c2 => Ok(token.token),
            (
                Token::Number(n1),
                Some(
                    token @ LocatingToken {
                        token: Token::Number(n2),
                        ..
                    },
                ),
            ) if *n1 == n2 => Ok(token.token),
            (
                Token::Word(w1),
                Some(
                    token @ LocatingToken {
                        token: Token::Word(w2),
                        ..
                    },
                ),
            ) if *w1 == w2 => Ok(token.token),
            (
                Token::GroupOpen(g1),
                Some(
                    token @ LocatingToken {
                        token: Token::GroupOpen(g2),
                        ..
                    },
                ),
            ) if *g1 == g2 => Ok(token.token),
            (
                Token::GroupClose(g1),
                Some(
                    token @ LocatingToken {
                        token: Token::GroupClose(g2),
                        ..
                    },
                ),
            ) if *g1 == g2 => Ok(token.token),
            (
                Token::MathOpen,
                Some(LocatingToken {
                    token: Token::MathOpen,
                    ..
                }),
            ) => Ok(Token::MathOpen),
            (
                Token::MathClose,
                Some(LocatingToken {
                    token: Token::MathClose,
                    ..
                }),
            ) => Ok(Token::MathClose),
            (
                Token::SequenceOpen,
                Some(LocatingToken {
                    token: Token::SequenceOpen,
                    ..
                }),
            ) => Ok(Token::SequenceOpen),
            (
                Token::SequenceClose,
                Some(LocatingToken {
                    token: Token::SequenceClose,
                    ..
                }),
            ) => Ok(Token::SequenceClose),
            (_, Some(token)) => Err(TypstError::from_token(&token)),
            (_, None) => Err(TypstError::from_input(input)),
        }
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
            WordFallbackParser::default()
                .with_taken()
                .parse_next(&mut &text[pos - *start..])
                .map(|(token, v)| LocatingToken {
                    token,
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
            } => parser(text, &self.pos(), start),
            PreToken::Token { token, .. } => Some(LocatingToken {
                token: Token::Raw(token),
                offset: self.pos(),
                len: 1,
            }),
            PreToken::MathOpen => Some(LocatingToken {
                token: Token::MathOpen,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::MathClose => Some(LocatingToken {
                token: Token::MathClose,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::SequenceOpen => Some(LocatingToken {
                token: Token::SequenceOpen,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::SequenceClose => Some(LocatingToken {
                token: Token::SequenceClose,
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
        self.len
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
