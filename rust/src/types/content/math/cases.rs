use crate::{Array, Boolean, Relative, types::content::math::generic::Delim};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Cases<'a> {
    #[serde(borrow)]
    pub delim: Delim<'a>,
    pub reverse: Boolean,
    pub gap: Relative,
    #[serde(borrow)]
    pub children: Array<'a>,
}

crate::impl_all_content!(Cases<'a>.Math, "math.cases");