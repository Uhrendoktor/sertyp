use winnow::{
    error::{AddContext, ParserError},
    stream::Stream,
};

use crate::{
    Box, Color, Content, FillColor, Length, Locatable, LocatingSequence, Or, Panic, Place, RBox,
    Sequence, Stroke, Text, TypedItem, Underline, math::Equation,
    types::content::sequence::winnow::PreToken,
};

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
        msg.push(Text::from_string(format!(": {}\n", expected)).into());
    }
    if !found.is_empty() {
        msg.push(Text::from_string("found").bold().into());
        msg.push(Text::from_string(format!(": {}\n", found)).into());
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
                    dy: Some(TypedItem::new(Length::pt(3.0).into())),
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
                                        Color::rgba_hex("#dc3545").unwrap(),
                                    )),
                                    ..Default::default()
                                })),
                                evade: Some(TypedItem(false.into())),
                                extent: Some(TypedItem(Length::pt(1.5))),
                                body: Some(RBox::new(TypedItem(
                                    Box {
                                        fill: Some(Or::Right(FillColor::Color(
                                            Color::rgba_hex("#fdecea").unwrap(),
                                        ))),
                                        inset: Some(Or::Left(Length::pt(1.0).into())),
                                        radius: Some(Or::Left(Length::pt(2.0).into())),
                                        body: Some(TypedItem(error).into()),
                                        ..Box::default()
                                    }
                                    .into(),
                                ))),
                                background: Some(TypedItem(true.into())),
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
