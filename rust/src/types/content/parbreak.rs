use crate::Content;
#[allow(unused_imports)]
use crate::Sequence;

/// Used within typst's internals to represent a space within a `Sequence`. Spaces are automatically inserted between different kinds of content items. See [Sequence] for more information.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Parbreak;

crate::impl_all!(Content<'a>::Parbreak, Parbreak {}, "parbreak");
