use crate::{
    Fraction, Or, Relative, TypedItem,
    types::{boolean::Boolean, content::Content},
};

/// For more information visit the typst documentation: [v](https://typst.app/docs/reference/layout/v/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct V {
    pub amount: Or<Relative, Fraction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak: Option<TypedItem<Boolean>>,
}

impl Default for V {
    fn default() -> Self {
        Self {
            amount: Or::Left(Relative::default()),
            weak: None,
        }
    }
}

crate::impl_all!(Content<'a>::V, V {}, "v");
