use crate::{Alignment, Array, Relative, types::content::math::generic::Delim};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Vec<'a> {
    #[serde(borrow)]
    pub delim: Option<Delim<'a>>,
    pub align: Alignment,
    pub gap: Relative,
    #[serde(borrow)]
    pub children: Array<'a>,
}

crate::impl_all_content!(Vec<'a>.Math, "math.vec");
