use winnow::{
    error::{AddContext, ParserError},
    stream::Stream,
};

use crate::{
    Box, Color, Content, FillColor, GroupType, Length, Locatable, LocatingSequence, Or, Panic,
    Place, RBox, Sequence, Stroke, Text, TypedItem, Underline,
    math::Equation,
    types::content::{sequence::winnow::PreToken, text::TextWeight},
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
        msg.push(
            Text::from_string("expected")
                .weight(TextWeight::Bold)
                .into(),
        );
        msg.push(Text::from_string(format!(": {}\n", expected)).into());
    }
    if !found.is_empty() {
        msg.push(Text::from_string("found").weight(TextWeight::Bold).into());
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

/// Wraps the content into a nicely highlighted inline error
/// - light red box
/// - red underline
pub fn inline_error<'a>(body: Content<'a>) -> Content<'a> {
    Underline {
        stroke: Some(Or::Right(Stroke {
            paint: Or::Right(FillColor::Color(Color::rgba_hex("#dc3545").unwrap())),
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
                body: Some(TypedItem::new(body).into()),
                ..Box::default()
            }
            .into(),
        ))),
        background: Some(TypedItem(true.into())),
        ..Default::default()
    }
    .into()
}

impl<'a> TypstError<'a> {
    /// Reconstructs a `Sequence` and injects visual error annotations.
    ///
    /// # Behavior
    /// - Preserves original structure (`sequence`, `math`, raw/content tokens).
    /// - Underlines the error span and highlights it in red using [`inline_error`].
    /// - Inserts [`error_box`] after the highlighted span.
    pub fn render(&self, sequence: &LocatingSequence<'a>) -> Sequence<'a> {
        let mut rendered = Sequence::new();

        // stack for traversing up the sequence tree whenever a sequence is closed
        type Stack<'a> = Vec<*mut Sequence<'a>>;
        let mut stack = vec![&mut rendered as *mut Sequence<'a>];

        // helper to get current sequence (top of stack)
        unsafe fn cur<'a, 'b>(stack: &'b mut Stack<'a>) -> &'b mut Sequence<'a> {
            unsafe { &mut **stack.last().unwrap() }
        }

        // adds content to the current sequence (top of stack)
        fn push<'a>(stack: &mut Stack<'a>, content: Content<'a>) {
            unsafe { cur(stack) }.push(content);
        }

        // adds an object to the current sequence. this object contains a nested sequence which is set as the new current sequence (pushed to stack)
        fn open_group_and_push<'a>(
            stack: &mut Stack<'a>,
            content: Content<'a>,
            f: impl for<'b> Fn(&'b mut Content<'a>) -> &'b mut Sequence<'a>,
        ) {
            push(stack, content);
            let seq_ptr = f(unsafe { cur(stack) }.last_mut().unwrap()) as *mut Sequence<'a>;
            stack.push(seq_ptr);
        }

        fn close_group(stack: &mut Stack<'_>) {
            stack.pop();
        }

        // opens an error box group if not already open.
        let mut error_present = false;
        fn try_open_error<'a>(
            stack: &mut Stack<'a>,
            error: &TypstError<'a>,
            error_present: &mut bool,
        ) {
            if !*error_present {
                push(stack, error_box(error));
                open_group_and_push(stack, inline_error(Sequence::new().into()), |c| match c {
                    Content::Underline(Underline {
                        body: Some(body), ..
                    }) => match &mut ***body {
                        Content::Box(b) => match &mut **b {
                            Box {
                                body: Some(body), ..
                            } => match &mut ***body {
                                Content::Sequence(seq) => seq,
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                });
                *error_present = true;
            }
        }

        // processes a PreToken
        fn process_token<'a>(stack: &mut Stack<'a>, token: &PreToken<'a>) {
            match token {
                PreToken::Token { token, .. } => push(stack, (*token).clone()),
                PreToken::Open(GroupType::Math) => {
                    open_group_and_push(
                        stack,
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
                }
                PreToken::Open(GroupType::Sequence) => {
                    open_group_and_push(stack, Sequence::new().into(), |content| match content {
                        Content::Sequence(seq) => seq,
                        _ => unreachable!(),
                    });
                }
                PreToken::Close(_) => {
                    close_group(stack);
                }
            }
        }

        let mut i = 0;
        while let Some((range, token)) = sequence.tokens.get_key_value(&i) {
            match (range, (self.offset..self.offset + self.len)) {
                // token is not covered by error span
                (t, e) if t.end <= e.start || t.start >= e.end => {
                    process_token(&mut stack, token);
                }
                // token is fully covered by error span (also single token non text)
                (t, e) if t.start >= e.start && t.end <= e.end => {
                    try_open_error(&mut stack, self, &mut error_present);
                    process_token(&mut stack, token);
                    // end error
                    if e.end == t.end {
                        close_group(&mut stack);
                    }
                }
                // token is partially covered by error span
                (t, e) => {
                    let text = match token {
                        PreToken::Token {
                            token: Content::Text(text),
                            ..
                        } => &**text,
                        _ => panic!("expected text token if token has length > 0"),
                    };
                    // split into three parts: Option<a> | b | Option<c> where b is the part covered by the error span

                    // split token at error end if neccessary
                    let (a, b, c) = text.slice_at(
                        e.start.saturating_sub(t.start),
                        e.end.saturating_sub(t.start),
                    );

                    if let Some(a) = a {
                        push(&mut stack, a.into());
                    };
                    // open error box for b
                    try_open_error(&mut stack, self, &mut error_present);
                    push(&mut stack, b.into());

                    // close even if c is 0 chars long
                    if e.end <= t.end {
                        close_group(&mut stack);
                    }
                    if let Some(c) = c {
                        push(&mut stack, c.into());
                    }
                }
            }

            // jump to next token
            i = range.end;
        }

        if !error_present {
            // error was not rendered, probably because it was on an invisible token. render the entire sequence as error.
            return Sequence::from(vec![error_box(self), inline_error(rendered.into())]);
        }

        rendered
    }
}
