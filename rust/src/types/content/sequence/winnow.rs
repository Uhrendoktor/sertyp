use core::panic;
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut, Range},
};

use winnow::{
    Parser, Result,
    ascii::{float, multispace0},
    combinator::{alt, delimited, not, peek},
    error::{AddContext, ContextError, EmptyError, ParserError, StrContext, StrContextValue},
    stream::{Offset, Stream, StreamIsPartial},
    token::{one_of, take_while},
};

use crate::{Content, Raw, Sequence, Space, TypedContent, TypedItem, math::Equation};

impl<'a, T: TryFrom<Content<'a>>> Parser<&'a Sequence<'a>, T, ContextError> for TypedContent<T>
where
    <T as TryFrom<Content<'a>>>::Error: std::fmt::Display,
{
    fn parse_next(&mut self, input: &mut &'a Sequence<'a>) -> winnow::Result<T, ContextError> {
        let checkpoint = input.as_slice().checkpoint();
        match input.as_slice().next_token() {
            Some(content) => match content.try_into() {
                Ok(typed_item) => Ok(typed_item),
                Err(err) => Err(ContextError::from_input(&input.as_slice())
                    .add_context(
                        &input.as_slice(),
                        &checkpoint,
                        StrContext::Label("TryFrom conversion"),
                    )
                    .add_context(
                        &input.as_slice(),
                        &checkpoint,
                        // This has to leak to create 'static lifetime
                        StrContext::Expected(StrContextValue::Description(Box::leak(
                            err.to_string().into_boxed_str(),
                        ))),
                    )),
            },
            None => Err(ParserError::from_input(&input.as_slice())),
        }
    }
}

// TODO: Outsource to own crate

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
    /// indexing is done by [PreToken] index
    lookup: Vec<usize>,
    /// indexing is done by inbetween-token offsets. Each character in a string has a offset
    tokens: rangemap::RangeMap<usize, PreToken<'a>>,
}
impl<'a> PreTokenMap<'a> {
    fn new() -> Self {
        PreTokenMap {
            lookup: vec![],
            tokens: rangemap::RangeMap::new(),
        }
    }
    fn insert(&mut self, range: std::ops::Range<usize>, token: PreToken<'a>) {
        self.lookup.push(range.start);
        self.tokens.insert(range, token);
    }
    fn get_by_offset(&self, offset: &usize) -> Option<&PreToken<'a>> {
        self.tokens.get(offset)
    }
    fn get_by_token(&self, token: &usize) -> Option<&PreToken<'a>> {
        let offset = self.lookup.get(*token)?;
        self.tokens.get(offset)
    }
}

#[derive(Debug, Clone)]
pub struct LocatingSequence<'a: 'b, 'b> {
    /// unparsed tokens represented by [Content]
    tokens: Cow<'b, PreTokenMap<'a>>,
    pos: usize,
    len: usize,
    offset: usize,
}
pub struct TypstError {
    message: String,
    span: Range<usize>,
}
impl<'a> From<&'a Sequence<'a>> for LocatingSequence<'a, '_> {
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
                    tokens.insert(
                        *pos..*pos + len,
                        PreToken::Token {
                            token: content,
                            start: *pos,
                            len,
                        },
                    );
                    *pos += len;
                }
                Content::Raw(Raw {
                    text: TypedItem(text),
                    ..
                }) => {
                    let len: usize = text.len();
                    tokens.insert(
                        *pos..*pos + len,
                        PreToken::Token {
                            token: content,
                            start: *pos,
                            len,
                        },
                    );
                    *pos += len;
                }
                Content::Sequence(seq) => {
                    tokens.insert(*pos..*pos + 1, PreToken::SequenceOpen);
                    *pos += 1;
                    insert_sequence(pos, seq, tokens);
                    tokens.insert(*pos..*pos + 1, PreToken::SequenceClose);
                    *pos += 1;
                }
                Content::MathEquation(Equation { body, .. }) => {
                    tokens.insert(*pos..*pos + 1, PreToken::MathOpen);
                    *pos += 1;
                    insert_content(pos, body, tokens);
                    tokens.insert(*pos..*pos + 1, PreToken::MathClose);
                    *pos += 1;
                }
                c => {
                    tokens.insert(
                        *pos..*pos + 1,
                        PreToken::Token {
                            token: c,
                            start: *pos,
                            len: 1,
                        },
                    );
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
            tokens: Cow::Owned(tokens),
            pos: 0,
            len: pos,
            offset: 0,
        }
    }
}

impl Offset for Pos {
    fn offset_from(&self, start: &Self) -> usize {
        self.0 - start.0
    }
}
impl<'a> Offset<Pos> for LocatingSequence<'a, '_> {
    /// Offset is calculated as the number of tokens
    fn offset_from(&self, start: &Pos) -> usize {
        self.pos - **start
    }
}

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Raw(&'a Content<'a>),
    Delimiter(char),
    Number(f32),
    Word(&'a str),
    GroupOpen(char),
    GroupClose(char),
    MathOpen,
    MathClose,
    SequenceOpen,
    SequenceClose,
}
impl<'a> Iterator for LocatingSequence<'a, '_> {
    type Item = (usize, Token<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        // next_token increments pos
        self.next_token().map(|token| (self.pos, token))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos(usize);
impl Deref for Pos {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Pos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<usize> for Pos {
    fn from(value: usize) -> Self {
        Pos(value)
    }
}
static SPACE: Content = Content::Space(Space);
impl<'a: 'b, 'b> LocatingSequence<'a, 'b> {
    pub fn lookahead(&'b self) -> LocatingSequence<'a, 'b> {
        LocatingSequence {
            tokens: Cow::Borrowed(&self.tokens),
            pos: self.pos,
            len: self.len,
            offset: self.offset,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos - self.offset
    }

    fn peek_len(&self) -> Option<(Token<'a>, usize)> {
        if self.pos() >= self.len {
            return None;
        }
        fn parse_text<'a>(text: &'a str, pos: &usize, start: &usize) -> Option<(Token<'a>, usize)> {
            fn whitespace_parser<'a>(input: &mut &'a str) -> winnow::Result<Token<'a>, EmptyError> {
                if input.chars().next().is_some_and(|c| c.is_whitespace()) {
                    *input = input.trim_start();
                    return Ok(Token::Raw(&SPACE));
                }
                Err(EmptyError)
            }
            fn parser<'a>(input: &mut &'a str) -> winnow::Result<Token<'a>, EmptyError> {
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
                    one_of([';', ':', ',', '.', '…', '·', '•', '@', '#', '$'])
                        .map(Token::Delimiter),
                    one_of([
                        '(', '[', '{', '<', '«', '‹', '“', '‘', '„', '‚', '⟨', '⟪', '⟮', '〈', '⌈',
                        '⌊', '⦇', '⦃', '⦅',
                    ])
                    .map(Token::GroupOpen),
                    one_of([
                        ')', ']', '}', '>', '»', '›', '”', '’', '‟', '‛', '⟩', '⟫', '⟯', '〉', '⌉',
                        '⌋', '⦈', '⦄', '⦆',
                    ])
                    .map(Token::GroupClose),
                    one_of(['|', '¦', '‖']).map(Token::Delimiter),
                    one_of(['\\', '`', '\'', '"', '_']).map(Token::Delimiter),
                    whitespace_parser,
                ))
                .parse_next(input)
            }

            fn parser2<'a>(input: &mut &'a str) -> winnow::Result<Token<'a>, EmptyError> {
                let mut i = 0;
                loop {
                    match parser.parse_peek(&input[i..]) {
                        Ok(token) if i == 0 => {
                            *input = &input[i..];
                            return parser.parse_next(input);
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
            parser2
                .with_taken()
                .parse_next(&mut &text[pos - *start..])
                .map(|(token, v)| (token, v.len()))
                .ok()
        }

        match self.tokens.get_by_offset(&self.pos())? {
            PreToken::Token {
                token: Content::Text(text),
                start,
                ..
            } => parse_text(text.as_string(), &self.pos(), start),
            PreToken::Token {
                token: Content::Raw(Raw { text, .. }),
                start,
                ..
            } => parse_text(text, &self.pos(), start),
            PreToken::Token { token, .. } => Some((Token::Raw(token), 1)),
            PreToken::MathOpen => Some((Token::MathOpen, 1)),
            PreToken::MathClose => Some((Token::MathClose, 1)),
            PreToken::SequenceOpen => Some((Token::SequenceOpen, 1)),
            PreToken::SequenceClose => Some((Token::SequenceClose, 1)),
        }
    }
}
impl<'a> Stream for LocatingSequence<'a, '_> {
    type Token = Token<'a>;
    type Slice = Self;
    type IterOffsets = Self;
    type Checkpoint = Pos;

    fn iter_offsets(&self) -> Self::IterOffsets {
        self.clone()
    }

    fn eof_offset(&self) -> usize {
        self.len - self.pos()
    }

    fn next_token(&mut self) -> Option<Self::Token> {
        if let Some((token, len)) = self.peek_len() {
            self.pos += len; // Move to the next token
            return Some(token);
        }
        None
    }

    fn peek_token(&self) -> Option<Self::Token> {
        self.peek_len().map(|(token, _)| token)
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
        // TODO: not entirely correct, since pretoken != token
        self.tokens
            .lookup
            .get(tokens)
            .cloned()
            .ok_or(winnow::error::Needed::Unknown)
    }

    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        let slice = self.peek_slice(offset);
        self.len = offset;
        slice
    }

    fn peek_slice(&self, offset: usize) -> Self::Slice {
        match offset {
            _ if offset >= self.len => {
                panic!("Offset out of bounds: offset {}, len {}", offset, self.len)
            }
            _ if offset < self.pos() => panic!(
                "Offset must be greater than or equal to current position: offset {}, pos {}",
                offset,
                self.pos()
            ),
            _ => (),
        };
        LocatingSequence {
            // ideally this would be a Cow::Borrowed like lookahead, but the trait signature does not allow for this
            tokens: self.tokens.clone(),
            pos: offset,
            len: self.len - offset,
            offset: self.offset + offset,
        }
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.pos.into()
    }

    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        self.pos = **checkpoint;
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

impl<'a: 'b, 'b> StreamIsPartial for LocatingSequence<'a, 'b> {
    type PartialState = bool;

    fn complete(&mut self) -> Self::PartialState {
        true
    }

    fn restore_partial(&mut self, _state: Self::PartialState) {}

    fn is_partial_supported() -> bool {
        false
    }
}
