use std::fmt::Display;

use crate::{Length, Or, Ratio, types::generic::TypedArray};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Relative(
    pub Or<TypedArray<RelativeItem>, RelativeItem>,
);

crate::impl_all!(Relative, "relative");

crate::auto_impl!{
    #[derive(Clone, Debug)]
    pub enum RelativeItem {
        Length(Length),
        Ratio(Ratio),
    }
}

impl Display for Relative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {// join with +
            Or::Left(s) => {
                let repr = s.iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<_>>()
                    .join(" + ");
                write!(f, "{}", repr)
            },
            Or::Right(s) => write!(f, "{:?}", s),
        }    
    }
}