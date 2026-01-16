use crate::{Alignment, Array, Dictionary, Integer, Or, Relative, types::content::math::generic::Delim};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Mat<'a> {
    #[serde(borrow)]
    pub delim: Option<Delim<'a>>,
    pub align: Alignment,
    #[serde(borrow)]
    pub augment: Option<Or<Integer, Dictionary<'a>>>,
    pub gap: Relative,
    #[serde(rename = "row-gap")]
    pub row_gap: Relative,
    #[serde(rename = "column-gap")]
    pub column_gap: Relative,
    #[serde(borrow)]
    pub rows: Array<'a>,
}

crate::impl_all_content!(Mat<'a>.Math, "math.mat");
