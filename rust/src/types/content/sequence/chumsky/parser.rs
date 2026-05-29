use std::ops::Neg;

use crate::GroupType;
use crate::error::TypstError;
use crate::{
    Content, H, LocatingSequence, Pagebreak, Parbreak, Space, Text, V,
    chumsky::{LocatingSequenceLike, Token},
};
use chumsky::IterParser;
use chumsky::extra::ParserExtra;
use chumsky::label::LabelError;
use chumsky::recursive::recursive;
use chumsky::{
    Parser,
    primitive::{choice, just, one_of},
    select,
};

pub static MINUS: [char; 2] = ['-', '−'];
pub static MULTIPLY: [char; 3] = ['*', '×', '⋅'];

pub fn as_token<const N: usize>(c: &[char; N]) -> [Token<'static, 'static>; N] {
    std::array::from_fn(|i| super::Token::Char(c[i]))
}

/// Parses a single ascii digit character of the given radix
/// e.g. for radix 10, it parses '0'..='9', for radix 16, it parses '0'..='9', 'a'..='f', 'A'..='F'
/// # Args
///   - `radix`: The radix (base) to check the digit against (e.g. 10 for decimal, 16 for hexadecimal)
pub fn digit<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>(
    radix: u32,
) -> impl Parser<'this, I, char, E>
where
    E::Error: LabelError<'this, I, L>,
{
    select! {
        Token::Char(c) if c.is_digit(radix) => c,
    }
    .labelled(crate::String::from("digit").into())
}

/// Parses 1.. [`digit`]s in a row and collects them into a string
/// # Args
///   - `radix`: see [`digit`]
pub fn digits<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>(
    radix: u32,
) -> impl Parser<'this, I, String, E>
where
    E::Error: LabelError<'this, I, L>,
{
    digit(radix)
        .repeated()
        .at_least(1)
        .collect()
        .labelled(crate::String::from("digits").into())
}

/// Parses a sign character: +, -
/// # Note
/// there are multiple unicode variants each
pub fn sign<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>() -> impl Parser<'this, I, char, E>
where
    E::Error: LabelError<'this, I, L>,
{
    select! {
        Token::Char(c) if MINUS.contains(&c) || c == '+' => c,
    }
    .labelled(crate::String::from("sign").into())
}

pub fn character<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>(
    c: char,
) -> impl Parser<'this, I, char, E>
where
    E::Error: LabelError<'this, I, L>,
{
    just(Token::Char(c))
        .to(c)
        .labelled(crate::String::from(format!("character('{}')", c)).into())
}

/// Parses a specific word
/// # Args
///   - `word`: The word to parse (e.g. "inf", "nan", "e", "pi")
pub fn word<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>(
    word: &str,
) -> impl Parser<'this, I, String, E>
where
    E::Error: LabelError<'this, I, L>,
{
    just(
        word.chars()
            .map(Token::Char)
            .collect::<Vec<Token<'this, 'data>>>(),
    )
    .to(word.to_owned())
    .labelled(crate::String::from(format!("word(\"{}\")", word)).into())
}

/// Parses 1.. [`digit`]s in a row and tries to convert them into an integer of type `I`
/// # Args
///  - `radix`: see [`digit`]
pub fn unsigned_integer_no_radix<
    'this,
    'data: 'this,
    N: num_traits::Num,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>(
    radix: u32,
) -> impl Parser<'this, I, N, E>
where
    E::Error: From<TypstError<'data>>,
    E::Error: LabelError<'this, I, L>,
{
    digits(radix)
        .try_map(move |s, span| {
            N::from_str_radix(&s, radix).map_err(|_| {
                TypstError::full(
                    span,
                    "Integer Parsing Error",
                    format!("integer of radix {radix}"),
                    s,
                )
                .into()
            })
        })
        .labelled(crate::String::from(format!("unsigned integer of radix {radix}")).into())
}

/// Parses an unsigned float of type `F`
/// # Args
/// - `radix`: see [`digit`]
/// # Formats
/// - <digits> (e.g. 123)
/// - <digits>.<digits>? (e.g. 123.456, 123.)
/// - .<digits> (e.g. .456)
/// - <mantissa><exponent>? (e.g. 1e10, 1.5e-3, .5E+2)
pub fn unsigned_float_no_radix_no_naninf<
    'this,
    'data: 'this,
    F: num_traits::Num + Clone,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>(
    radix: u32,
) -> impl Parser<'this, I, F, E>
where
    E::Error: From<TypstError<'data>>,
    E::Error: LabelError<'this, I, L>,
{
    // Mantissa:
    let mantissa = choice((
        // <digits>
        digits(radix).map(|whole| (Some(whole), None)),
        // <digits>.<digits>?
        digits(radix)
            .then(just(Token::Char('.')))
            .then(digits(radix).or_not())
            .map(|((whole, _), decimals)| (Some(whole), decimals)),
        // .<digits>
        just(Token::Char('.'))
            .then(digits(radix))
            .map(|(_, decimals)| (None, Some(decimals))),
    ))
    .labelled(crate::String::from(format!("float mantissa of radix {radix}")).into());

    // Exponent:
    // (e|E)|(p|P) [+|-]? digits (the exponent indicator is e/E for radix <14, p/P for radix >= 14)
    let exponent = if radix < 14 {
        one_of([Token::Char('e'), Token::Char('E')])
    } else {
        one_of([Token::Char('p'), Token::Char('P')])
    }
    .map(|t| match t {
        Token::Char(c) => c,
        _ => unreachable!(),
    })
    .then(sign().or_not())
    .then(digits(radix))
    .or_not()
    .labelled(crate::String::from(format!("float exponent of radix {radix}")).into());

    // <mantissa><exponent>?
    mantissa
        .then(exponent.or_not())
        .try_map(move |((whole, decimals), exponent), span| {
            let f = format!(
                "{}.{}{}",
                whole.unwrap_or("0".to_string()),
                decimals.unwrap_or("0".to_string()),
                exponent
                    .flatten()
                    .map(|((e, sign), exponent)| format!("{e}{}{exponent}", sign.unwrap_or('+')))
                    .unwrap_or("".to_string())
            )
            .to_string();
            F::from_str_radix(&f, radix).map_err(|_| {
                TypstError::full(
                    span,
                    "Float Parsing Error",
                    format!("float of radix {radix}"),
                    f,
                )
                .into()
            })
        })
        .labelled(
            crate::String::from(format!(
                "unsigned float of radix {radix} without inf or nan"
            ))
            .into(),
        )
}

/// Parses an unsigned float of type `F` with the same formats as [`unsigned_float_no_radix_no_naninf`] but also allows for "inf" and "nan" (case-insensitive) and for radix prefixes (0b, 0o, 0x)
/// # Format
/// - see [`unsigned_float_no_radix_no_naninf`]
/// - inf | INF
/// - nan | NAN | NaN
/// - 0b<digits> (binary float)
/// - 0o<digits> (octal float)
/// - 0x<digits> (hexadecimal float, uses 'p' or 'P' as exponent indicator instead of 'e' or 'E')
pub fn unsigned_float_no_radix<
    'this,
    'data: 'this,
    F: num_traits::Num + From<f32> + Clone,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>(
    radix: u32,
) -> impl Parser<'this, I, F, E>
where
    E::Error: From<TypstError<'data>>,
    E::Error: LabelError<'this, I, L>,
{
    choice((
        unsigned_float_no_radix_no_naninf(radix),
        // inf | INF
        word("inf").or(word("INF")).to(f32::INFINITY.into()),
        // nan | NAN
        word("nan")
            .or(word("NAN"))
            .or(word("NaN"))
            .to(f32::NAN.into()),
    ))
    .labelled(
        crate::String::from(format!(
            "unsigned float of radix {radix} with optional inf and nan"
        ))
        .into(),
    )
}

pub fn auto_radix<
    'this,
    'data: 'this,
    T,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    P: Parser<'this, I, T, E>,
    F: Fn(u32) -> P,
    L: From<crate::String<'data>> + Clone,
>(
    parser: F,
    default_radix: u32,
) -> impl Parser<'this, I, T, E>
where
    E::Error: LabelError<'this, I, L>,
{
    choice((
        parser(default_radix),
        word("0b").then(parser(2)).map(|(_, t)| t),
        word("0o").then(parser(8)).map(|(_, t)| t),
        word("0x").then(parser(16)).map(|(_, t)| t),
    ))
    .labelled(
        crate::String::from(format!(
            "radix detection (0b, 0o, 0x or nothing={default_radix})"
        ))
        .into(),
    )
}

/// Wraps a parser with signing logic.
///
/// # Format
/// - <parser>
/// - +<parser>
/// - -<parser> (invokes `Neg` trait)
pub fn signed<
    'this,
    'data: 'this,
    N: Neg<Output = N>,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
    P: Parser<'this, I, N, E>,
    L: From<crate::String<'data>> + Clone,
>(
    parser: P,
) -> impl Parser<'this, I, N, E>
where
    E::Error: LabelError<'this, I, L>,
{
    sign()
        .or_not()
        .then(parser)
        .map(|(sign, num)| {
            if let Some(sign) = sign
                && MINUS.contains(&sign)
            {
                num.neg()
            } else {
                num
            }
        })
        .labelled(crate::String::from("signed").into())
}

/// Prases a variable name.
/// # Format
/// - <non-digit><digit|non-digit>* (e.g. a, a1, a_2, _a, _1a, but not 1a)
/// - subscript or superscript variables (e.g. x₁, ²y) (powers are NOT treated as part of the variable name)
/// - accent variables (e.g. x̄, ȳ)
pub fn variable<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, LocatingSequence<'this, 'data>> + ParserExtra<'this, I>,
    L: From<crate::String<'data>> + Clone,
>() -> impl Parser<'this, I, Content<'data>, E>
where
    <E as ParserExtra<'this, I>>::Error: From<TypstError<'data>>,
    <E as ParserExtra<'this, LocatingSequence<'this, 'data>>>::Error: From<TypstError<'data>>,
    <E as ParserExtra<'this, LocatingSequence<'this, 'data>>>::Error: std::fmt::Debug,
    <E as ParserExtra<'this, LocatingSequence<'this, 'data>>>::Context: Default,
    <E as ParserExtra<'this, LocatingSequence<'this, 'data>>>::State: Default,
    <E as ParserExtra<'this, I>>::Error: LabelError<'this, I, L>,
    <E as ParserExtra<'this, LocatingSequence<'this, 'data>>>::Error:
        LabelError<'this, LocatingSequence<'this, 'data>, L>,
{
    choice((
        // normal string based variable name
        select! {Token::Char(c) if c.is_alphabetic() || c == '_' => c}
        .then(
            select! {Token::Char(c) if c.is_alphabetic() || c.is_ascii_digit() || c == '_' => c}
                .repeated()
                .collect::<String>(),
        )
        .map(|(prefix, mut remaining)| {
            remaining.insert(0, prefix);
            remaining
        }).map(|s| Text::from_string(s).into()).labelled(crate::String::from("string variable name").into()),

        // subscript or superscript variable name
        select!{ Token::Raw(Content::MathAttach(attach)) => attach }.try_map(|attach, span| {
            macro_rules! field {
                ($field:expr) => {{
                    variable::<LocatingSequence<'this, 'data>, E, L>()
                        .parse(<LocatingSequence as From<&'this Content<'data>>>::from($field))
                        .into_result()
                        .map(|_| ())
                        .map_err(|errors| {
                            return TypstError::full(
                                span,
                                "Invalid Variable Subscript/Superscript",
                                "Variable like declaration in attachment",
                                format!("non variable like attachment in {}: {:?}", stringify!($field), errors.first().expect("Expected at least one error")),
                             ).into()
                        })
                }};
            }
            macro_rules! maybe_field  {
                ($field:expr) => {
                    if let Some(inner) = $field {
                        field!(&***inner)
                    } else {
                        Ok(())
                    }
                };
            }
            field!(&**attach.base)?;
            maybe_field!(&attach.b)?;
            maybe_field!(&attach.bl)?;
            maybe_field!(&attach.br)?;
            // t is NOT checked since it is used for exponents
            maybe_field!(&attach.tl)?;
            maybe_field!(&attach.tr)?;
            Ok(Content::MathAttach(attach.clone()))
        }).labelled(crate::String::from("subscript or superscript variable").into()),
        select!{ Token::Raw(Content::MathAccent(accent)) => accent }.try_map(|accent, span| {
            variable::<LocatingSequence<'this, 'data>, E, L>()
                .parse(<LocatingSequence as From<&'this Content<'data>>>::from(&**accent.base))
                .into_result()
                .map(|_| ())
                .map_err(|errors| {
                    TypstError::full(
                        span,
                        "Invalid Variable Accent",
                        "Variable like declaration in accent",
                        format!("non variable like attachment in accent: {:?}", errors.first().expect("Expected at least one error")),
                     )
                })?;
            Ok(Content::MathAccent(accent.clone()))
        }).labelled(crate::String::from("accent variable").into())
    )).labelled(crate::String::from("variable").into())
}

#[derive(Debug, Clone)]
pub enum Whitespace<'this, 'data> {
    Space(&'this Space),
    Parbreak(&'this Parbreak),
    Pagebreak(&'this Pagebreak<'data>),
    V(&'this V),
    H(&'this H),
}
/// Parses a single whitespace item (space, parbreak, pagebreak, v, h)
pub fn whitespace<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
>() -> impl Parser<'this, I, Whitespace<'this, 'data>, E> {
    choice((
        select! {Token::Raw(Content::Space(s)) => s}.map(Whitespace::Space),
        select! {Token::Raw(Content::Parbreak(p)) => p}.map(Whitespace::Parbreak),
        select! {Token::Raw(Content::Pagebreak(pb)) => pb}.map(Whitespace::Pagebreak),
        select! {Token::Raw(Content::V(v)) => v}.map(Whitespace::V),
        select! {Token::Raw(Content::H(h)) => h}.map(Whitespace::H),
    ))
}
/// Parses 0.. [`whitespace`]s in a row and collects them into a vector
/// Can be used to ignore whitespace between other tokens
pub fn whitespaces<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    E: ParserExtra<'this, I>,
>() -> impl Parser<'this, I, Vec<Whitespace<'this, 'data>>, E> {
    whitespace().repeated().collect()
}

pub fn delimited_by_groups<
    'this,
    'data: 'this,
    I: LocatingSequenceLike<'this, 'data>,
    O: 'this,
    E: 'this + ParserExtra<'this, I>,
    L: 'this + From<crate::String<'data>> + Clone,
>(
    atom: impl 'this + Parser<'this, I, O, E> + Clone,
) -> impl Parser<'this, I, O, E>
where
    E::Error: LabelError<'this, I, L>,
{
    recursive(|parser| {
        choice((
            atom,
            parser.clone().delimited_by(
                just(Token::Open(GroupType::Math)),
                just(Token::Close(GroupType::Math)),
            ),
            parser.clone().delimited_by(
                just(Token::Open(GroupType::Sequence)),
                just(Token::Close(GroupType::Sequence)),
            ),
        ))
    })
    .labelled(crate::String::from("delimited by groups").into())
}

pub trait Number {
    fn parser<
        'this,
        'data: 'this,
        I: LocatingSequenceLike<'this, 'data>,
        E: chumsky::extra::ParserExtra<'this, I>,
        L: From<crate::String<'data>> + Clone,
    >() -> impl Parser<'this, I, Self, E>
    where
        Self: Sized,
        E::Error: std::convert::From<TypstError<'data>>,
        E::Error: LabelError<'this, I, L>;
}

macro_rules! number {
    (float) => {
        auto_radix(unsigned_float_no_radix, 10)
    };
    (int) => {
        auto_radix(|radix| unsigned_integer_no_radix(radix), 10)
    };
    ($n:ident $ty:ty) => {
        impl Number for $ty {
            fn parser<
                'this,
                'data: 'this,
                I: LocatingSequenceLike<'this, 'data>,
                E: chumsky::extra::ParserExtra<'this, I>,
                L: From<crate::String<'data>> + Clone,
            >() -> impl Parser<'this, I, Self, E>
            where
                E::Error: std::convert::From<TypstError<'data>>,
                E::Error: LabelError<'this, I, L>,
            {
                number!($n)
            }
        }
    };
    (signed $n:ident $ty:ty) => {
        impl Number for $ty {
            fn parser<
                'this,
                'data: 'this,
                I: LocatingSequenceLike<'this, 'data>,
                E: chumsky::extra::ParserExtra<'this, I>,
                L: From<crate::String<'data>> + Clone,
            >() -> impl Parser<'this, I, Self, E>
            where
                E::Error: std::convert::From<TypstError<'data>>,
                E::Error: LabelError<'this, I, L>,
            {
                signed(number!($n))
            }
        }
    };
}

number!(int u8);
number!(int u16);
number!(int u32);
number!(int u64);
number!(int usize);
number!(signed int i8);
number!(signed int i16);
number!(signed int i32);
number!(signed int i64);
number!(signed int isize);
number!(float f32);
number!(float f64);
