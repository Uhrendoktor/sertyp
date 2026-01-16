use crate::{Content, Direction, Fraction, Or, Relative, TypedItem, impl_all_content, types::generic::TypedArray};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Stack<'a> {
    #[serde(skip_serializing_if="Option::is_none")]
    pub dir: Option<TypedItem<Direction>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub spacing: Option<Or<Relative, Fraction>>,
    #[serde(borrow)]
    pub children: TypedArray<StackChildren<'a>>
}

impl_all_content!(Stack<'a>, "stack");

crate::auto_impl!{
    #[derive(Debug, Clone)]
    pub enum StackChildren<'a> {
        Relative(Relative),
        Fraction(Fraction),
        Content(Content<'a>),
    }
}