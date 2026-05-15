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
    error::{AddContext, ContextError, ParserError},
    stream::{Compare, Offset, ParseSlice, Stream, StreamIsPartial},
    token::one_of,
};

use crate::{
    Box, CharStream, Color, Content, Length, Or, Panic, Place, RBox, Raw, Sequence, Space, Stroke,
    Symbol, Text, TypedItem, Underline, float,
    math::{Equation, LR},
    types::generic::FillColor,
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
    <I as Stream>::Slice: ParseSlice<f32>,
    <I as Stream>::IterOffsets: Clone,
{
    fn parse_next(&mut self, input: &mut I) -> Result<Token<'a>, E> {
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
#[derive(Debug, Clone)]
pub struct TypstError<'a> {
    /// Global token/character offset where parsing failed.
    pub offset: usize,
    /// Span length of the failing token/segment.
    pub len: usize,
    /// Underlying winnow context error.
    pub inner: Vec<Context<'a>>,
}
impl<'a> TypstError<'a> {
    /// Appends a context entry to the underlying error and returns `self`.
    pub fn context(mut self, context: Context<'a>) -> Self {
        self.inner.push(context);
        self
    }

    /// Creates an error positioned at `token`.
    pub fn from_token<T>(token: &Locatable<T>) -> Self {
        TypstError {
            offset: token.offset,
            len: token.len,
            inner: vec![],
        }
    }

    pub fn manual(offset: usize, len: usize) -> Self {
        TypstError {
            offset,
            len,
            inner: vec![],
        }
    }
}
impl<'a> ParserError<LocatingSequence<'a>> for TypstError<'a> {
    type Inner = Self;

    fn assert(input: &LocatingSequence<'a>, message: &'static str) -> Self
    where
        LocatingSequence<'a>: core::fmt::Debug,
    {
        TypstError::from_input(input).context(Context::Label(message.into()))
    }

    fn from_input(input: &LocatingSequence<'_>) -> Self {
        TypstError {
            offset: input.global_pos(),
            len: 1,
            inner: vec![],
        }
    }

    fn into_inner(self) -> Result<Self::Inner, Self> {
        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub enum Context<'a> {
    Label(crate::String<'a>),
    Expected(crate::String<'a>),
    Found(crate::String<'a>),
}
impl<'a> AddContext<LocatingSequence<'a>, Context<'a>> for TypstError<'a> {
    #[inline]
    fn add_context(
        mut self,
        _input: &LocatingSequence<'a>,
        _token_start: &<LocatingSequence<'a> as Stream>::Checkpoint,
        context: Context<'a>,
    ) -> Self {
        self.inner.push(context);
        self
    }
}

/// Builds an absolute positioned floating Typst error. Similar to a normal [Panic].
pub fn error_box<'a>(error: &TypstError<'a>) -> Content<'a> {
    let expression: Option<_> = error.inner.iter().find_map(|c| match c {
        Context::Label(c) => Some(c),
        _ => None,
    });
    macro_rules! filter_variant {
        ($variant:ident) => {
            error
                .inner
                .iter()
                .filter_map(|c| match c {
                    Context::$variant(c) => Some(c.to_string()),
                    _ => None,
                })
                .collect::<Vec<String>>()
                .join(", ")
        };
    }
    let expected = filter_variant!(Expected);
    let found = filter_variant!(Found);
    let mut msg = vec![];
    if !expected.is_empty() {
        msg.push(Text::from_string("expected").bold().into());
        msg.push(Text::from_string(expected).into());
        if !found.is_empty() {
            msg.push(Text::from_string(", ").into());
        }
    }
    if !found.is_empty() {
        msg.push(Text::from_string("found").bold().into());
        msg.push(Text::from_string(found).into());
    }

    Content::Box(
        Box {
            body: Some(RBox(
                TypedItem::new(Content::Place(Place {
                    body: Some(
                        TypedItem::new(Content::Panic(Panic {
                            ty: format!("invalid {}", expression.unwrap_or(&"<unknown>".into()))
                                .into(),
                            msg: Content::from(Sequence::from(msg)).into(),
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
impl<'a> TypstError<'a> {
    /// Reconstructs a `Sequence` and injects visual error annotations.
    ///
    /// # Behavior
    /// - Preserves original structure (`sequence`, `math`, raw/content tokens).
    /// - Underlines the error span in red.
    /// - Inserts [`error_box`] after the highlighted span.
    pub fn render(&self, sequence: &LocatingSequence<'a>) -> Sequence<'a> {
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

        let mut hit = false;
        let mut offset = 0;
        while let Some(token) = sequence.tokens.get(&offset) {
            match token {
                PreToken::Token {
                    token, start, len, ..
                } => {
                    if (*start <= self.offset) && (self.offset < start + len) {
                        hit = true;
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

        // special case if error was thrown on invisible token
        if !hit {
            return Sequence {
                children: vec![
                    Underline {
                        stroke: Some(Or::Right(Stroke {
                            paint: Or::Right(FillColor::Color(Color::rgba_hex("#FF0000").unwrap())),
                            ..Default::default()
                        })),
                        evade: Some(TypedItem(false.into())),
                        extent: Some(TypedItem(Length::pt(1.5))),
                        body: Some(RBox::new(TypedItem(seq.into()))),
                        ..Default::default()
                    }
                    .into(),
                    error_box(self),
                ]
                .into(),
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
                Content::MathLR(LR { body, .. }) => {
                    insert_content(pos, body, tokens);
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
impl<'a> Parser<LocatingSequence<'a>, Self, TypstError<'a>> for Token<'a> {
    fn parse_next(&mut self, input: &mut LocatingSequence<'a>) -> Result<Self, TypstError<'a>> {
        match (self, input.next_token()) {
            (
                Token::Delimiter(c1),
                Some(
                    token @ LocatingToken {
                        inner: Token::Delimiter(c2),
                        ..
                    },
                ),
            ) => {
                if *c1 == c2 {
                    Ok(token.inner)
                } else {
                    Err(TypstError::from_token(&token)
                        .context(Context::Label("delimiter".into()))
                        .context(Context::Expected((*c1).to_string().into()))
                        .context(Context::Found(c2.to_string().into())))
                }
            }
            (
                Token::Number(n1),
                Some(
                    token @ LocatingToken {
                        inner: Token::Number(n2),
                        ..
                    },
                ),
            ) => {
                if *n1 == n2 {
                    Ok(token.inner)
                } else {
                    Err(TypstError::from_token(&token)
                        .context(Context::Label("number".into()))
                        .context(Context::Expected((*n1).to_string().into()))
                        .context(Context::Found(n2.to_string().into())))
                }
            }
            (
                Token::Word(w1),
                Some(
                    token @ LocatingToken {
                        inner: Token::Word(w2),
                        ..
                    },
                ),
            ) => {
                if *w1 == w2 {
                    Ok(token.inner)
                } else {
                    Err(TypstError::from_token(&token)
                        .context(Context::Label("word".into()))
                        .context(Context::Expected((*w1).to_string().into()))
                        .context(Context::Found(w2.to_string().into())))
                }
            }
            (
                Token::GroupOpen(g1),
                Some(
                    token @ LocatingToken {
                        inner: Token::GroupOpen(g2),
                        ..
                    },
                ),
            ) => {
                if *g1 == g2 {
                    Ok(token.inner)
                } else {
                    Err(TypstError::from_token(&token)
                        .context(Context::Label("group open".into()))
                        .context(Context::Expected((*g1).to_string().into()))
                        .context(Context::Found(g2.to_string().into())))
                }
            }
            (
                Token::GroupClose(g1),
                Some(
                    token @ LocatingToken {
                        inner: Token::GroupClose(g2),
                        ..
                    },
                ),
            ) => {
                if *g1 == g2 {
                    Ok(token.inner)
                } else {
                    Err(TypstError::from_token(&token)
                        .context(Context::Label("group close".into()))
                        .context(Context::Expected((*g1).to_string().into()))
                        .context(Context::Found(g2.to_string().into())))
                }
            }
            (
                Token::MathOpen,
                Some(LocatingToken {
                    inner: Token::MathOpen,
                    ..
                }),
            ) => Ok(Token::MathOpen),
            (Token::MathOpen, t) => Err(TypstError::from_input(input)
                .context(Context::Label("math open".into()))
                .context(Context::Found(format!("{t:#?}").into()))),
            (
                Token::MathClose,
                Some(LocatingToken {
                    inner: Token::MathClose,
                    ..
                }),
            ) => Ok(Token::MathClose),
            (Token::MathClose, t) => Err(TypstError::from_input(input)
                .context(Context::Label("math close".into()))
                .context(Context::Found(format!("{t:#?}").into()))),
            (
                Token::SequenceOpen,
                Some(LocatingToken {
                    inner: Token::SequenceOpen,
                    ..
                }),
            ) => Ok(Token::SequenceOpen),
            (Token::SequenceOpen, t) => Err(TypstError::from_input(input)
                .context(Context::Label("sequence open".into()))
                .context(Context::Found(format!("{t:#?}").into()))),
            (
                Token::SequenceClose,
                Some(LocatingToken {
                    inner: Token::SequenceClose,
                    ..
                }),
            ) => Ok(Token::SequenceClose),
            (Token::SequenceClose, t) => Err(TypstError::from_input(input)
                .context(Context::Label("sequence close".into()))
                .context(Context::Found(format!("{t:#?}").into()))),
            (t, Some(token)) => Err(TypstError::from_token(&token)
                .context(Context::Label("parser".into()))
                .context(Context::Expected(format!("{t:#?}").into()))
                .context(Context::Found(format!("{token:#?}").into()))),
            (t, None) => Err(TypstError::from_input(input)
                .context(Context::Label("parser".into()))
                .context(Context::Expected(format!("{t:#?}").into()))
                .context(Context::Found("end of input".into()))),
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
            } => parser(text, &self.pos(), start),
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
            PreToken::MathOpen => Some(LocatingToken {
                inner: Token::MathOpen,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::MathClose => Some(LocatingToken {
                inner: Token::MathClose,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::SequenceOpen => Some(LocatingToken {
                inner: Token::SequenceOpen,
                offset: self.pos(),
                len: 1,
            }),
            PreToken::SequenceClose => Some(LocatingToken {
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
