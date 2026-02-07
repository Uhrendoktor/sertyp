use crate::{Content, Direction, Fraction, Or, Relative, TypedItem, types::generic::TypedArray};

/// For more information visit the typst documentation: [stack](https://typst.app/docs/reference/layout/stack/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Stack<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<TypedItem<Direction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Or<Relative, Fraction>>,
    #[serde(borrow)]
    pub children: TypedArray<StackChildren<'a>>,
}

crate::impl_all!(Content<'a>::Stack, Stack<'a>{'a}, "stack");

crate::auto_impl! {
    #[derive(Debug, Clone, Hash)]
    pub enum StackChildren<'a> {
        try_from{},
        Relative(Relative=>Relative),
        Fraction(Fraction=>Fraction),
        Content(Content=>std::boxed::Box<Content<'a>>),
    }
}

impl<'a> Default for StackChildren<'a> {
    fn default() -> Self {
        StackChildren::Content(Content::default().into())
    }
}
