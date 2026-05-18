use crate::Content;
#[allow(unused_imports)]
use crate::Sequence;

/// For more information visit the typst documentation: [parbreak](https://typst.app/docs/reference/model/parbreak/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Parbreak;

crate::impl_all!(Content<'a>::Parbreak, Parbreak {}, "parbreak");
