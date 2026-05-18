#[allow(unused_imports)]
use crate::Sequence;
use crate::{Boolean, Content, NoneOr, String, TypedItem};

/// For more information visit the typst documentation: [pagebreak](https://typst.app/docs/reference/layout/pagebreak/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Pagebreak<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    weak: Option<TypedItem<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    to: Option<NoneOr<String<'a>>>,
}

crate::impl_all!(Content<'a>::Pagebreak, Pagebreak<'a> {'a}, "pagebreak");
