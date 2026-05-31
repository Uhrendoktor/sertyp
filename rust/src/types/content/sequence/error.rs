use std::mem::transmute;

use chumsky::span::{SimpleSpan, Span};

use crate::{
    Box, Color, Content, FillColor, GroupType, Length, LocatingSequence, Or, Panic, Place,
    PreToken, Sequence, Stroke, Text, TypedItem, Underline, math::Equation,
    types::content::text::TextWeight,
};

/// A parser error annotated with a span in the locating stream.
///
/// `TypstError` is the data structure that `LocatingSequence` ultimately feeds.
/// The span is expressed in flattened offsets, so it can be replayed against the
/// original content tree to reconstruct the affected region precisely.
#[derive(Debug, Clone)]
pub struct TypstError<'data, S: Span = SimpleSpan> {
    pub span: S,
    /// Underlying winnow context error.
    pub inner: Vec<Context<'data>>,
}
impl<'a, S: Span> TypstError<'a, S> {
    /// Appends a context entry to the underlying error and returns `self`.
    pub fn context(mut self, context: Context<'a>) -> Self {
        self.inner.push(context);
        self
    }

    pub fn context_mut(&mut self, context: Context<'a>) -> &mut Self {
        self.inner.push(context);
        self
    }

    /// Creates an error positioned at `token`.
    pub fn spanned(span: S) -> Self {
        TypstError {
            span,
            inner: vec![],
        }
    }

    pub fn full(
        span: S,
        label: impl Into<crate::String<'a>>,
        expected: impl Into<crate::String<'a>>,
        found: impl Into<crate::String<'a>>,
    ) -> Self {
        TypstError {
            span,
            inner: vec![
                Context::Label(label.into()),
                Context::Expected(expected.into()),
                Context::Found(found.into()),
            ],
        }
    }
}

impl<'a> From<Vec<TypstError<'a>>> for TypstError<'a> {
    fn from(mut value: Vec<TypstError<'a>>) -> Self {
        value.remove(0)
    }
}

/// Human-readable context entries attached to a `TypstError`.
///
/// These values are collected while parsing and are later turned into the
/// rendered error box. The `Label` variant usually identifies the expression or
/// construct currently being validated, while `Expected` and `Found` are used
/// to summarize mismatch details.
#[derive(Debug, Clone)]
pub enum Context<'data> {
    Label(crate::String<'data>),
    Expected(crate::String<'data>),
    Found(crate::String<'data>),
}

/// Builds an absolute positioned floating Typst error. Similar to a normal [Panic].
pub fn error_box<'data, S: Span>(error: &TypstError<'data, S>) -> Content<'data> {
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
            body: Some(std::boxed::Box::new(TypedItem::new(Content::Place(
                Place {
                    body: Some(
                        TypedItem::new(Content::Panic(Panic {
                            ty: expression.cloned().unwrap_or("<unknown>".into()).into(),
                            msg: Content::from(Sequence::from(msg)).into(),
                        }))
                        .into(),
                    ),
                    dy: Some(TypedItem::new(Length::pt(3.0).into())),
                    dx: Some(TypedItem::new(Length::pt(-20.0).into())),
                    ..Default::default()
                },
            )))),
            ..Default::default()
        }
        .into(),
    )
}

/// Wraps the content into a nicely highlighted inline error
/// - light red box
/// - red underline
pub fn inline_error<'data>(body: Content<'data>) -> Content<'data> {
    Underline {
        stroke: Some(Or::Right(Stroke {
            paint: Or::Right(FillColor::Color(Color::rgba_hex("#dc3545").unwrap())),
            ..Default::default()
        })),
        evade: Some(TypedItem(false.into())),
        extent: Some(TypedItem(Length::pt(1.5))),
        body: Some(std::boxed::Box::new(TypedItem(
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

impl<'data, S: Span<Offset = usize>> TypstError<'data, S> {
    /// Reconstructs a `Sequence` and injects visual error annotations.
    ///
    /// # Behavior
    /// - Preserves original structure (`sequence`, `math`, raw/content tokens).
    /// - Underlines the error span and highlights it in red using [`inline_error`].
    /// - Inserts [`error_box`] after the highlighted span.
    pub fn render<'this>(
        &'this self,
        sequence: &'this LocatingSequence<'this, 'data>,
    ) -> Sequence<'data> {
        let mut rendered: Sequence<'data> = Sequence::new();

        // stack for traversing up the sequence tree whenever a sequence is closed
        type Stack = Vec<*mut Sequence<'static>>;
        let mut stack: Stack = vec![unsafe {
            transmute::<*mut Sequence<'data>, *mut Sequence<'static>>(
                &mut rendered as *mut Sequence<'data>,
            )
        }];

        // helper to get current sequence (top of stack)
        unsafe fn cur<'this, 'data>(stack: &'this mut Stack) -> &'this mut Sequence<'data> {
            unsafe {
                &mut *transmute::<*mut Sequence<'static>, *mut Sequence<'data>>(
                    *stack.last().unwrap(),
                )
            }
        }

        // adds content to the current sequence (top of stack)
        fn push<'this, 'data>(stack: &'this mut Stack, content: Content<'data>) {
            unsafe { cur(stack) }.push(content);
        }

        // adds an object to the current sequence. this object contains a nested sequence which is set as the new current sequence (pushed to stack)
        fn open_group_and_push<'this, 'data>(
            stack: &'this mut Stack,
            content: Content<'data>,
            f: impl for<'this2> Fn(&'this2 mut Content<'data>) -> &'this2 mut Sequence<'data>,
        ) {
            push(stack, content);
            let seq_ptr = unsafe {
                transmute::<*mut Sequence<'data>, *mut Sequence<'static>>(f(cur(stack)
                    .last_mut()
                    .unwrap())
                    as *mut Sequence<'data>)
            };
            stack.push(seq_ptr);
        }

        fn close_group(stack: &mut Stack) {
            stack.pop();
        }

        // opens an error box group if not already open.
        let mut error_present = false;
        fn try_open_error<'this, 'data, S: Span>(
            stack: &'this mut Stack,
            error: &'this TypstError<'data, S>,
            error_present: &'this mut bool,
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
        fn process_token<'this, 'data>(
            stack: &'this mut Stack,
            token: &'this PreToken<'this, 'data>,
        ) {
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
            match (range, (self.span.start()..self.span.end())) {
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
