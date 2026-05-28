use std::ops::Range;

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
pub enum PreToken<'this, 'data: 'this> {
    Token {
        token: &'this Content<'data>,
        start: usize,
        #[allow(unused)]
        len: usize,
    },
    Open(GroupType),
    Close(GroupType),
}
/// Impls pointer compare for PreTokens, as equals is only used when inserting Tokens into the rangemap.
impl<'this, 'data> PartialEq for PreToken<'this, 'data> {
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
impl<'this, 'data> Eq for PreToken<'this, 'data> {}

pub type PreTokenMap<'this, 'data> = rangemap::RangeMap<usize, PreToken<'this, 'data>>;

pub fn insert_token<'this, 'data>(
    map: &mut PreTokenMap<'this, 'data>,
    content: &'this Content<'data>,
    range: Range<usize>,
) {
    map.insert(
        range.clone(),
        PreToken::Token {
            token: content,
            start: range.start,
            len: range.end - range.start,
        },
    );
}
pub fn get<'this, 'data>(
    map: &'data PreTokenMap<'this, 'data>,
    offset: &usize,
) -> Option<&'data PreToken<'this, 'data>> {
    map.get(offset)
}

#[derive(Debug, Clone)]
/// Stream adapter over `Sequence` with stable position tracking.
///
/// # Purpose
/// - Expose sequence content as a winnow `Stream`.
/// - Preserve token offsets for precise parser errors.
/// - Support lookahead and slicing without losing global positions.
pub struct LocatingSequence<'this, 'data: 'this> {
    /// unparsed tokens represented by [Content]
    pub tokens: PreTokenMap<'this, 'data>,
    /// current position.
    pub pos: usize,
    /// cummulative length or [PreToken]s. [Text] contribute their respective number of [char]s, everything else has length=1.
    pub len: usize,
    /// starting position in relation to indices used in [PreTokenMap]. Can be unequal to 0 in cases where sub-sequences are created (lookahead, etc.).
    pub offset: usize,
}

impl<'this, 'data> LocatingSequence<'this, 'data> {
    /// Returns the length of the token stream in terms of offsets.
    /// NOT in terms of the actual number of [`PreTokens`].
    pub fn len(&self) -> usize {
        self.tokens
            .last_range_value()
            .map(|(k, _)| k.end)
            .unwrap_or(0)
    }
}

fn insert_content<'this, 'data>(
    pos: &mut usize,
    content: &'this Content<'data>,
    tokens: &mut PreTokenMap<'this, 'data>,
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
fn insert_sequence<'this, 'data>(
    pos: &mut usize,
    seq: &'this Sequence<'data>,
    tokens: &mut PreTokenMap<'this, 'data>,
) {
    for content in seq.as_slice().iter() {
        insert_content(pos, content, tokens);
    }
}

impl<'this, 'data> From<&'this Sequence<'data>> for LocatingSequence<'this, 'data> {
    fn from(seq: &'this Sequence<'data>) -> Self {
        let mut tokens = PreTokenMap::new();
        let mut pos = 0;
        insert_sequence(&mut pos, seq, &mut tokens);
        LocatingSequence {
            tokens,
            pos: 0,
            len: pos,
            offset: 0,
        }
    }
}

impl<'this, 'data> From<&'this Content<'data>> for LocatingSequence<'this, 'data> {
    fn from(content: &'this Content<'data>) -> Self {
        let mut tokens = PreTokenMap::new();
        let mut pos = 0;
        insert_content(&mut pos, content, &mut tokens);
        LocatingSequence {
            tokens,
            pos: 0,
            len: pos,
            offset: 0,
        }
    }
}
