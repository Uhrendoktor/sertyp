use std::ops::{Deref, DerefMut};

use crate::{Content, Item, types::generic::TypedArray};

/// Used within typst's internals to represent a space seperated sequence of different content items within a single content block. This is basically an array of `Content`.
/// # Example of Typst Behavior
/// ```typst
/// #let content = [a sentence with some math: $a+b=c$]
/// // is parsed as `sequence(([a sentence with some math] [:] space math.equation(...))`
/// ```
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Sequence<'a> {
    #[serde(borrow)]
    pub children: TypedArray<Content<'a>>,
}

crate::impl_into!(Content<'a>::Sequence, Sequence<'a>);
crate::impl_typst_type!(Sequence<'a>{'a}, "sequence");

impl<'a> From<Content<'a>> for Sequence<'a> {
    fn from(content: Content<'a>) -> Self {
        match content {
            Content::Sequence(seq) => seq,
            other => Sequence {
                children: TypedArray::from(vec![other]),
            },
        }
    }
}

/// Utility wrapper that enable auto serialization and deserialization for any type T that implements [TryFrom]<[Sequence]> and [Into]<[Sequence]>.
#[derive(Debug, Clone)]
pub struct TypedSequence<T>(pub T);

impl<T> Deref for TypedSequence<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TypedSequence<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> TypedSequence<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'a, T: TryFrom<Sequence<'a>>> TryFrom<Sequence<'a>> for TypedSequence<T> {
    type Error = T::Error;

    fn try_from(value: Sequence<'a>) -> std::result::Result<Self, Self::Error> {
        Ok(TypedSequence(value.try_into()?))
    }
}

impl<'a, T: TryFrom<Sequence<'a>>> TryFrom<Content<'a>> for TypedSequence<T> {
    type Error = T::Error;

    fn try_from(content: Content<'a>) -> std::result::Result<Self, Self::Error> {
        let seq: Sequence<'a> = content.into();
        seq.try_into()
    }
}

impl<'a, T: TryFrom<Sequence<'a>, Error = <Content<'a> as TryFrom<Item<'a>>>::Error>>
    TryFrom<Item<'a>> for TypedSequence<T>
{
    type Error = T::Error;

    fn try_from(item: Item<'a>) -> std::result::Result<Self, Self::Error> {
        let content: Content = item.try_into()?;
        content.try_into()
    }
}

impl<'a, T: Into<Sequence<'a>>> From<TypedSequence<T>> for Sequence<'a> {
    fn from(typed_seq: TypedSequence<T>) -> Self {
        typed_seq.0.into()
    }
}

impl<'a, T: Into<Sequence<'a>>> From<TypedSequence<T>> for Content<'a> {
    fn from(typed_seq: TypedSequence<T>) -> Self {
        Sequence::from(typed_seq).into()
    }
}

impl<'a, T: Into<Sequence<'a>>> From<TypedSequence<T>> for Item<'a> {
    fn from(typed_seq: TypedSequence<T>) -> Self {
        Content::from(typed_seq).into()
    }
}

#[macro_export]
macro_rules! auto_impl_sequence {
    (
        $vis:vis struct $name:ident($($visf:vis $tyf:ty: $idf:tt $([$delim:tt])?),* $(,)?);
    ) => {
        auto_impl_sequence! {$vis struct $name ['a] ( $($visf $tyf: $idf $([$delim])?),* );}
    };
    (
        $vis:vis struct $name:ident$(<$($g:tt),*>)? $([$($g2:tt),*])?($($visf:vis $tyf:ty: $idf:tt $([$delim:tt])?),* $(,)?);
    ) => {
        $vis struct $name$(<$($g),*>)?($( $visf $tyf ),*);

        impl<$($($g)*,)? $($($g2),*)?> TryFrom<$crate::StructuredSequence<'a>> for $name$(<$($g),*>)? {
            type Error = std::string::String;

            fn try_from(mut seq: $crate::StructuredSequence<'a>) -> std::result::Result<Self, Self::Error> {
                let v = $name(
                    $(
                        {
                            let $crate::ParsedSequence { value: v, remaining } = <$tyf as $crate::SequenceView>::parse(seq)?;
                            seq = remaining;
                            $(
                                {
                                    struct DelimiterParser(pub char);
                                    impl $crate::SequenceView for DelimiterParser {
                                        fn parse<'a>(
                                            $crate::StructuredSequence { children }: $crate::StructuredSequence<'a>,
                                        ) -> std::result::Result<$crate::ParsedSequence<'a, Self>, std::string::String> {
                                            match children.first() {
                                                Some($crate::structured_sequence::Element::Delimiter(t)) if &**t == stringify!($delim) => {
                                                    Ok($crate::ParsedSequence {
                                                        value: DelimiterParser(stringify!($delim).chars().next().unwrap()),
                                                        remaining: $crate::StructuredSequence {
                                                            children: children[1..].to_vec().into(),
                                                        },
                                                    })
                                                },
                                                _ => Err(format!("Expected delimiter '{}' in sequence", stringify!($delim)).to_string()),
                                            }
                                        }
                                    }
                                    let $crate::ParsedSequence { remaining, .. } = DelimiterParser::parse(seq)?;
                                    seq = remaining;
                                }
                            )?
                            v
                        },
                    )*
                );
                if !seq.children.is_empty() {
                    return Err("Extra elements found in sequence".to_string());
                }
                Ok(v)
            }
        }

        impl<$($($g)*,)? $($($g2),*)?> TryFrom<$crate::Sequence<'a>> for $name$(<$($g),*>)? {
            type Error = <$crate::StructuredSequence<'a> as TryFrom<$crate::Sequence<'a>>>::Error;

            fn try_from(seq: $crate::Sequence<'a>) -> std::result::Result<Self, Self::Error> {
                let structured_seq: $crate::StructuredSequence<'a> = seq.try_into()?;
                structured_seq.try_into()
            }
        }

        impl<$($($g)*,)? $($($g2),*)?> From<$name$(<$($g),*>)?> for $crate::StructuredSequence<'a> {
            fn from(val: $name$(<$($g),*>)?) -> Self {
                let mut v = vec![];
                $({
                    let seq: StructuredSequence = val.$idf.into();
                    v.extend(seq.children);
                    $(
                        v.push($crate::structured_sequence::Element::Content(std::boxed::Box::new($crate::Text::from_string(stringify!($delim)).into())));
                    )?
                })*

                $crate::StructuredSequence {
                    children: v,
                }
            }
        }
    };
}

pub use structured_sequence::{
    ParsedSequence, Sequence as StructuredSequence, SequenceView, TypedStructuredSequence,
};
pub mod structured_sequence {

    use std::{
        collections::VecDeque,
        ops::{Deref, DerefMut},
    };

    use crate::{
        Content, Symbol, Text, impl_typst_type,
        math::{Equation, LR},
    };

    #[derive(Debug, Clone)]
    pub struct Sequence<'a> {
        pub children: Vec<Element<'a>>,
    }

    impl_typst_type!(typst_like Sequence<'a>{'a}, "structured-sequence");

    #[derive(Debug, Clone)]
    pub enum Element<'a> {
        Delimiter(Delim<'a>),
        Group {
            delim: GroupDelim<'a>,
            children: std::boxed::Box<Sequence<'a>>,
        },
        Content(std::boxed::Box<Content<'a>>),
    }

    impl<'a> From<Element<'a>> for Content<'a> {
        fn from(value: Element<'a>) -> Self {
            match value {
                Element::Delimiter(delim) => {
                    Content::Text(std::boxed::Box::new(Text::from_string(delim)))
                }
                Element::Group { delim, children } => {
                    if delim.opening() == '\0' || delim.closing() == '\0' {
                        return Content::Sequence(super::Sequence::from(*children));
                    }
                    let mut v = vec![];
                    v.push(Content::Text(std::boxed::Box::new(Text::from_string(
                        delim.0.to_string(),
                    ))));
                    v.extend(super::Sequence::from(*children).children);
                    v.push(Content::Text(std::boxed::Box::new(Text::from_string(
                        delim.1.to_string(),
                    ))));
                    Content::Sequence(super::Sequence { children: v.into() })
                }
                Element::Content(cont) => *cont,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct GroupDelim<'a>(pub Symbol<'a>, pub Symbol<'a>);
    impl<'a> GroupDelim<'a> {
        pub fn as_text(&self) -> Text<'a> {
            Text::from_string(format!("{}{}", self.0, self.1))
        }

        pub fn opening(&self) -> char {
            *self.0
        }

        pub fn closing(&self) -> char {
            *self.1
        }
    }

    pub type Delim<'a> = Symbol<'a>;

    impl<'a> Sequence<'a> {
        pub fn parse_from_sequence(
            seq: super::Sequence<'a>,
            group_delims: &[GroupDelim<'a>],
            delims: &[Delim<'a>],
        ) -> std::result::Result<Self, std::string::String> {
            let mut hirarchy = vec![('\0', vec![])];
            let mut group = &mut hirarchy[0];
            let closing = |c: char| -> char {
                group_delims
                    .iter()
                    .find(|gc| gc.opening() == c)
                    .map_or('\0', |gc| gc.closing())
            };
            let mut queue = seq.children.into_iter().collect::<VecDeque<_>>();
            while let Some(content) = queue.pop_front() {
                match content {
                    Content::Text(t) => {
                        let Text { text, .. } = *t;
                        for c in text.chars() {
                            match c {
                                _ if group_delims.iter().any(|gc| gc.opening() == c) => {
                                    hirarchy.push((c, vec![]));
                                    group = hirarchy.last_mut().unwrap();
                                }
                                _ if group_delims.iter().any(|gc| gc.closing() == c) => {
                                    if closing(group.0) != c {
                                        return Err(format!(
                                            "Mismatched closing delimiter: expected '{}' but found '{}'",
                                            closing(group.0),
                                            c
                                        ));
                                    }
                                    let (o_delim, children) = hirarchy.pop().unwrap();
                                    group = hirarchy.last_mut().unwrap();
                                    group.1.push(Element::Group {
                                        delim: GroupDelim(
                                            o_delim.to_string().into(),
                                            c.to_string().into(),
                                        ),
                                        children: std::boxed::Box::new(Sequence { children }),
                                    });
                                }
                                _ if delims.iter().any(|d| **d == c) => {
                                    group.1.push(Element::Delimiter(c.to_string().into()));
                                }
                                other => {
                                    if let Some(Element::Content(cont)) = group.1.last_mut()
                                        && let Content::Text(t) = cont.deref_mut()
                                    {
                                        let Text { text, .. } = t.as_mut();
                                        // copy Cow to owned if necessary
                                        (**text).0.to_mut().push(other);
                                    } else {
                                        group.1.push(Element::Content(Box::new(
                                            Text::from_string(c.to_string()).into(),
                                        )));
                                    }
                                }
                            }
                        }
                    }
                    Content::Parbreak(_) | Content::Space(_) | Content::H(_) => {
                        if let Some(Element::Content(cont)) = group.1.last_mut()
                            && let Content::Text(t) = cont.deref_mut()
                        {
                            let Text { text, .. } = t.as_mut();
                            // copy Cow to owned if necessary
                            (**text).0.to_mut().push(' ');
                        }
                        // ignore. WARNING: this will lead to wrong visual deserialization
                    }
                    Content::MathLR(LR { body, .. })
                    | Content::MathEquation(Equation { body, .. }) => {
                        queue.push_front(*body.0.0);
                    }
                    Content::Symbol(Symbol(symbol)) => {
                        queue.push_front(Text::from_string(symbol.to_string()).into());
                    }
                    Content::Sequence(super::Sequence { children }) => {
                        for child in children.into_iter().rev() {
                            queue.push_front(child);
                        }
                    }
                    other => {
                        group.1.push(Element::Content(Box::new(other)));
                    }
                }
            }
            if hirarchy.len() != 1 {
                return Err("Unclosed delimiter in sequence".to_string());
            }
            Ok(Sequence {
                children: hirarchy.pop().unwrap().1,
            })
        }
    }

    impl<'a> TryFrom<super::Sequence<'a>> for Sequence<'a> {
        type Error = std::string::String;

        fn try_from(value: super::Sequence<'a>) -> std::result::Result<Self, Self::Error> {
            Self::parse_from_sequence(
                value,
                &[
                    ('(', ')'),
                    ('[', ']'),
                    ('{', '}'),
                    ('⟨', '⟩'),
                    ('⟪', '⟫'),
                    ('<', '>'),
                ]
                .map(|(o, c)| GroupDelim(o.to_string().into(), c.to_string().into())),
                &[';', ':', ',', '|'].map(|c| c.to_string().into()),
            )
        }
    }

    impl<'a> From<Sequence<'a>> for super::Sequence<'a> {
        fn from(seq: Sequence<'a>) -> Self {
            super::Sequence {
                children: seq
                    .children
                    .into_iter()
                    .flat_map(|el| match el.into() {
                        Content::Sequence(super::Sequence { children }) => children.into_inner(),
                        other => vec![other],
                    })
                    .collect::<Vec<_>>()
                    .into(),
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct TypedStructuredSequence<T>(pub T);

    impl_typst_type!(typst_like TypedStructuredSequence<T>{T}, "typed-structured-sequence");

    impl<T> Deref for TypedStructuredSequence<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for TypedStructuredSequence<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T> TypedStructuredSequence<T> {
        pub fn into_inner(self) -> T {
            self.0
        }
    }

    pub struct ParsedSequence<'a, T> {
        pub value: T,
        pub remaining: Sequence<'a>,
    }

    impl_typst_type!(typst_like ParsedSequence<'a, T>{'a, T}, "parsed-sequence");

    pub trait SequenceView {
        fn parse<'a>(
            seq: Sequence<'a>,
        ) -> std::result::Result<ParsedSequence<'a, Self>, std::string::String>
        where
            Self: Sized;
    }

    impl<'a, T: SequenceView> TryFrom<Sequence<'a>> for TypedStructuredSequence<T> {
        type Error = std::string::String;

        fn try_from(seq: Sequence<'a>) -> std::result::Result<Self, Self::Error> {
            let ParsedSequence { value, remaining } = T::parse(seq)?;
            if !remaining.children.is_empty() {
                return Err("Extra elements found in sequence".to_string());
            }
            Ok(TypedStructuredSequence(value))
        }
    }

    impl<'a, T: SequenceView> TryFrom<super::Sequence<'a>> for TypedStructuredSequence<T> {
        type Error = <Sequence<'a> as TryFrom<super::Sequence<'a>>>::Error;

        fn try_from(seq: super::Sequence<'a>) -> std::result::Result<Self, Self::Error> {
            let seq = Sequence::try_from(seq)?;
            seq.try_into()
        }
    }

    impl<'a, T> From<ParsedSequence<'a, T>> for super::Sequence<'a>
    where
        T: Into<Sequence<'a>>,
    {
        fn from(seq: ParsedSequence<'a, T>) -> Self {
            seq.value.into().into()
        }
    }

    impl<'a, T: Into<Sequence<'a>>> From<TypedStructuredSequence<T>> for super::Sequence<'a> {
        fn from(typed_seq: TypedStructuredSequence<T>) -> Self {
            typed_seq.into_inner().into().into()
        }
    }
}
