use std::ops::Neg;

use winnow::{
    Parser,
    combinator::{alt, opt, seq},
    error::ParserError,
    stream::Stream,
    token::one_of,
};

use crate::{Content, Context, Locatable, LocatingSequence, LocatingToken, Token, TypstError};

pub fn ignore_groups<'a, T>(
    input: &mut LocatingSequence<'a>,
    mut parser: impl Parser<LocatingSequence<'a>, T, TypstError<'a>>,
) -> Result<T, TypstError<'a>> {
    let mut i = 0;
    while let Some(LocatingToken {
        inner: Token::MathOpen | Token::SequenceOpen,
        ..
    }) = input.peek_token()
    {
        input.next_token();
        i += 1;
    }
    let res = parser.parse_next(input)?;
    for _ in 0..i {
        match input.next_token() {
            Some(LocatingToken {
                inner: Token::MathClose | Token::SequenceClose,
                ..
            }) => {}
            Some(token) => {
                return Err(TypstError::from_token(&token)
                    .context(Context::Label("sequence simplification".into()))
                    .context(Context::Expected("closing parenthesis".into())));
            }
            None => {
                return Err(TypstError::from_input(input)
                    .context(Context::Label("sequence simplification".into()))
                    .context(Context::Expected("closing parenthesis".into())));
            }
        }
    }
    Ok(res)
}

pub fn ignore_whitespaces<'a>(input: &mut LocatingSequence<'a>) -> Result<(), TypstError<'a>> {
    while let Some(LocatingToken {
        inner: Token::Raw(Content::Space(_)),
        ..
    }) = input.peek_token()
    {
        input.next_token();
    }
    Ok(())
}

pub fn signed<
    'a,
    P: Parser<LocatingSequence<'a>, Locatable<T>, TypstError<'a>>,
    T: Neg<Output = T>,
>(
    input: &mut LocatingSequence<'a>,
    mut parser: P,
) -> Result<Locatable<T>, TypstError<'a>> {
    seq!(
        opt(one_of([
            Token::Delimiter('+'),
            Token::Delimiter('−'),
            Token::Delimiter('-')
        ])),
        parser
    )
    .parse_next(input)
    .map(|(sign, t)| match sign {
        Some(LocatingToken {
            inner: Token::Delimiter(s),
            offset,
            len,
        }) => Locatable {
            inner: match s {
                '+' => t.inner,
                '−' | '-' => t.inner.neg(),
                _ => unreachable!(),
            },
            offset: offset,
            len: len + t.len,
        },
        _ => t,
    })
}

pub fn int<'a, T: TryFrom<usize>>(
    input: &mut LocatingSequence<'a>,
) -> Result<Locatable<T>, TypstError<'a>>
where
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    match input.next_token() {
        Some(
            t @ Locatable {
                inner: Token::Number(n),
                ..
            },
        ) => {
            let num: T = n.try_into().map_err(|e| {
                TypstError::from_token(&t)
                    .context(Context::Label("integer".into()))
                    .context(Context::Expected(format!("{:#?}", e).into()))
            })?;
            Ok(t.map(|_| num))
        }
        _ => {
            return Err(TypstError::from_input(input)
                .context(Context::Label("integer".into()))
                .context(Context::Expected("integer".into())));
        }
    }
}

pub fn float<'a, F: From<f32>>(
    input: &mut LocatingSequence<'a>,
) -> Result<Locatable<F>, TypstError<'a>> {
    alt((
        seq!(int::<usize>, opt((Token::Delimiter('.'), opt(int)))).map(|(whole_t, decimals_t)| {
            let decimals = match &decimals_t {
                Some((_, Some(dec))) => dec.inner,
                _ => 0,
            };
            let whole = whole_t.inner;
            Locatable {
                inner: format!("{whole}.{decimals}").parse().unwrap(),
                offset: whole_t.offset,
                len: whole_t.len
                    + decimals_t
                        .as_ref()
                        .map(|(dot, dec)| dot.len + dec.as_ref().map(|dec| dec.len).unwrap_or(0))
                        .unwrap_or(0),
            }
        }),
        one_of([Token::Word("inf"), Token::Word("INF")])
            .map(|t: LocatingToken| t.map(|_| f32::INFINITY)),
        one_of([Token::Word("nan"), Token::Word("NAN"), Token::Word("NaN")])
            .map(|t: LocatingToken| t.map(|_| f32::NAN)),
    ))
    .parse_next(input)
    .map(|t| t.map(|f| f.into()))
}

pub trait Number: Sized {
    fn parse<'a>(input: &mut LocatingSequence<'a>) -> Result<Locatable<Self>, TypstError<'a>>;
}

macro_rules! impl_number {
    ($($s:ident $v:ident $t:ty),*) => {
        $(
            impl Number for $t {
                fn parse<'a>(input: &mut LocatingSequence<'a>) -> Result<Locatable<Self>, TypstError<'a>> {
                    $s(input, $v)
                }
            }
        )*
    };
}

// noop for macro
fn unsigned<'a, P: Parser<LocatingSequence<'a>, T, TypstError<'a>>, T>(
    input: &mut LocatingSequence<'a>,
    mut parser: P,
) -> Result<T, TypstError<'a>> {
    parser.parse_next(input)
}
impl_number! {
    signed int i8,
    signed int i16,
    signed int i32,
    signed int i64,
    signed int i128,
    signed int isize,
    signed float f32,
    signed float f64,
    unsigned int u8,
    unsigned int u16,
    unsigned int u32,
    unsigned int u64,
    unsigned int u128,
    unsigned int usize
}
