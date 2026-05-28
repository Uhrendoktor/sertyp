//! Flattened token streams with position tracking for Typst content.
//!
//! This module turns nested [`Sequence`](crate::Sequence) and [`Content`](crate::Content)
//! values into a linear token map that keeps enough structure for two consumers:
//! parser adapters and error rendering.
//!
//! The core idea is simple but easy to miss when reading the implementation:
//! text-like content advances by character width, while most other content
//! occupies one offset. Nested sequences and math bodies additionally emit
//! explicit open/close markers so downstream code can preserve grouping.
//!
//! The result is a stream that can be indexed by span position without losing
//! the original content boundaries.

use std::ops::Range;

use crate::{
    Content, Raw, Sequence, TypedItem,
    math::{Equation, LR},
};

/// Marks the kind of group boundary stored in the locating stream.
///
/// Only the outer structure matters here. `Sequence` covers nested content
/// blocks and `Math` covers equation bodies. Equality is intentionally based
/// on the enum discriminant rather than identity.
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

/// Internal token emitted while flattening content into offsets.
///
/// `PreToken` stores both structure and bookkeeping:
/// - `Token` points at the original `Content` and remembers its span.
/// - `Open` / `Close` bracket nested groups such as a `Sequence` or math body.
///
/// The type is public because the parser/error modules need to inspect it, but
/// it is an implementation detail of the locating pipeline rather than a user
/// facing abstraction.
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

/// Pointer-based equality for `PreToken`.
///
/// Equality is only needed for range-map operations, so token payloads are
/// compared by address. Group markers are compared by kind.
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

/// Range map keyed by flattened offsets.
///
/// Given an offset, the map tells you which `PreToken` covers that point.
/// This is the primary data structure used by parser adapters and error
/// reconstruction.
pub type PreTokenMap<'this, 'data> = rangemap::RangeMap<usize, PreToken<'this, 'data>>;

/// Insert one token into a locating map.
///
/// `range` is expressed in flattened offsets, not AST depth. For text this is
/// a character range. For most non-text nodes it is a single-unit range.
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

/// Look up the token covering `offset`.
pub fn get<'this, 'data>(
    map: &'data PreTokenMap<'this, 'data>,
    offset: &usize,
) -> Option<&'data PreToken<'this, 'data>> {
    map.get(offset)
}

/// Stream adapter over `Sequence` with stable position tracking.
///
/// `LocatingSequence` is the bridge between Typst content and parser/error
/// infrastructure. It flattens a nested content tree into a linear offset space
/// while keeping enough structure to reconstruct spans and group boundaries.
#[derive(Debug, Clone)]
pub struct LocatingSequence<'this, 'data: 'this> {
    /// Flattened token map keyed by offsets in the locating stream.
    pub tokens: PreTokenMap<'this, 'data>,
    /// Current cursor position.
    pub pos: usize,
    /// Flattened length in offsets.
    ///
    /// Text contributes its character count; all other tokens contribute one.
    pub len: usize,
    /// Offset base for subsequences or slices of a larger stream.
    pub offset: usize,
}

impl<'this, 'data> LocatingSequence<'this, 'data> {
    /// Return the total flattened length.
    ///
    /// This is intentionally offset-based rather than token-count-based so it
    /// matches the span arithmetic used by the parser and error renderer.
    pub fn len(&self) -> usize {
        self.tokens
            .last_range_value()
            .map(|(k, _)| k.end)
            .unwrap_or(0)
    }
}

/// Recursively flatten one content node into the locating map.
///
/// This function encodes the important invariants of the stream:
/// - `Text` and raw text advance by their string length.
/// - `Sequence` and math bodies emit explicit open/close markers.
/// - `MathLR` is transparent and forwards to its body.
/// - Everything else is treated as a single offset token.
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

/// Flatten an entire sequence by visiting each child in order.
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
    /// Build a locating stream from a `Sequence` reference.
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
    /// Build a locating stream from a single `Content` node.
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
