use std::fmt::Display;

use crate::{Item, Length, Ratio, types::generic::TypedArray};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Relative(
    pub TypedArray<RelativeItem>,
);

crate::impl_typst_type!(Relative, "relative");
crate::impl_into!(Relative);

impl<'a> TryFrom<Item<'a>> for Relative {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Relative(r) => Ok(r),
            Item::Ratio(r) => Ok(Relative(vec![RelativeItem::Ratio(r)].into())),
            Item::Length(l) => Ok(Relative(vec![RelativeItem::Length(l)].into())),
            other => Err(format!("Cannot convert {:?} into Relative", other)),
        }
    }
}

crate::auto_impl!{
    #[derive(Clone, Debug)]
    pub enum RelativeItem {
        Length(Length),
        Ratio(Ratio),
    }
}

impl Default for RelativeItem {
    fn default() -> Self {
        RelativeItem::Ratio(Ratio::default())
    }
}

impl Display for Relative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.0.iter()
            .map(|item| format!("{:?}", item))
            .collect::<Vec<_>>()
            .join(" + ");
        write!(f, "{}", repr)    
    }
}