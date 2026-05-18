use std::{ops::Range, rc::Rc, sync::atomic::AtomicUsize};

use crate::{
    Content, Raw, Sequence, TypedItem,
    math::{Equation, LR},
};

#[derive(Debug, Clone)]
pub enum GroupType {
    Sequence,
    Math,
    // TODO: impl
    // Raw(&'a Raw<'a>),
    // LR(&'a LR<'a>),
}

impl PartialEq for GroupType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // (GroupType::Raw(r1), GroupType::Raw(r2)) => std::ptr::eq(*r1, *r2),
            // (GroupType::LR(l1), GroupType::LR(l2)) => std::ptr::eq(*l1, *l2),
            (l, r) => std::mem::discriminant(l) == std::mem::discriminant(r),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PreToken<'a> {
    Token {
        token: &'a Content<'a>,
        start: usize,
        #[allow(unused)]
        len: usize,
    },
    Open(GroupType),
    Close(GroupType),
}
/// Impls pointer compare for PreTokens, as equals is only used when inserting Tokens into the rangemap.
impl<'a> PartialEq for PreToken<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PreToken::Token { token: t1, .. }, PreToken::Token { token: t2, .. }) => {
                std::ptr::eq(t1, t2)
            }
            (PreToken::Open(g1), PreToken::Open(g2)) => g1 == g2,
            (PreToken::Close(g1), PreToken::Close(g2)) => g1 == g2,
            _ => false,
        }
    }
}
impl<'a> Eq for PreToken<'a> {}

pub type PreTokenMap<'a> = rangemap::RangeMap<usize, PreToken<'a>>;
pub type PreTokenMapSlice<'a> = rangemap::RangeMap<usize, &'a PreToken<'a>>;

pub fn insert_token<'a>(map: &mut PreTokenMap<'a>, content: &'a Content<'a>, range: Range<usize>) {
    map.insert(
        range.clone(),
        PreToken::Token {
            token: content,
            start: range.start,
            len: range.end - range.start,
        },
    );
}
pub fn get<'a, 'b>(map: &'b PreTokenMap<'a>, offset: &usize) -> Option<&'b PreToken<'a>> {
    map.get(offset)
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
    pub tokens: PreTokenMap<'a>,
    /// shared reference of position.
    /// [Rc<AtomicUsize>] Neccessary for interior mutability and correct error reporting,
    /// as `iter_offsets` only provides a `&` reference.
    pub pos: Rc<AtomicUsize>,
    /// cummulative length or [PreToken]s. [Text] contribute their respective number of [char]s, everything else has length=1.
    pub len: usize,
    /// starting position in relation to indices used in [PreTokenMap]. Can be unequal to 0 in cases where sub-sequences are created (lookahead, etc.).
    pub offset: usize,
}

impl<'a> LocatingSequence<'a> {
    /// Returns the length of the token stream in terms of offsets.
    /// NOT in terms of the actual number of [`PreTokens`].
    pub fn len(&self) -> usize {
        self.tokens
            .last_range_value()
            .map(|(k, _)| k.end)
            .unwrap_or(0)
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
                    insert_token(tokens, content, *pos..*pos + len);
                    *pos += len;
                }
                Content::Raw(Raw {
                    text: TypedItem(text),
                    ..
                }) => {
                    let len: usize = text.len();
                    insert_token(tokens, content, *pos..*pos + len);
                    *pos += len;
                }
                Content::Sequence(seq) => {
                    tokens.insert(*pos..*pos + 1, PreToken::Open(GroupType::Sequence));
                    *pos += 1;
                    insert_sequence(pos, seq, tokens);
                    tokens.insert(*pos..*pos + 1, PreToken::Close(GroupType::Sequence));
                    *pos += 1;
                }
                Content::MathEquation(Equation { body, .. }) => {
                    tokens.insert(*pos..*pos + 1, PreToken::Open(GroupType::Math));
                    *pos += 1;
                    insert_content(pos, body, tokens);
                    tokens.insert(*pos..*pos + 1, PreToken::Close(GroupType::Math));
                    *pos += 1;
                }
                Content::MathLR(LR { body, .. }) => {
                    insert_content(pos, body, tokens);
                }
                content => {
                    insert_token(tokens, content, *pos..*pos + 1);
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
