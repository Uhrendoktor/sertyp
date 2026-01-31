use crate::{Content, types::generic::TypedArray};

/// Used within typst's internals to represent a space seperated sequence of different content items within a single content block. This is basically an array of `Content`.
/// # Example of Typst Behavior
/// ```typst
/// #let content = [a sentence with some math: $a+b=c$]
/// // is parsed as `sequence(([a sentence with some math] [:] space math.equation(...))`
/// ```
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Sequence<'a> {
    #[serde(borrow)]
    pub children: TypedArray<Content<'a>>,
}

crate::impl_all!(Content<'a>::Sequence, Sequence<'a>{'a}, "sequence");
