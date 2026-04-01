use std::fmt::Display;

use crate::{Item, Length, Ratio, types::generic::TypedArray};

/// Relative lengths are additive combinations of lengths and ratios.
/// ```typst
/// #let length = 10pt + 5% + 2pt + -3%;
/// ```
/// For single values, if typst flags a value as `relative` with a single value instead of `length` or `ratio` is arbitrary.
/// Therefore this type can auto case [Item] variants [Length] and [Ratio] into [Relative] when used in any context where [TryFrom]<[Item]> is required.
///
/// For more information visit the typst documentation: [relative](https://typst.app/docs/reference/layout/relative/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Relative(pub TypedArray<RelativeItem>);

crate::impl_typst_type!(Relative {}, "relative");
crate::impl_into!(Item<'a>::Relative, Relative);

impl<'a> TryFrom<Item<'a>> for Relative {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Relative(r) => Ok(r),
            Item::Ratio(r) => Ok(Relative(vec![RelativeItem::Ratio(r)].into())),
            Item::Length(l) => Ok(Relative(vec![RelativeItem::Length(l)].into())),
            other => Err(format!("Cannot convert {other:?} into Relative")),
        }
    }
}

crate::auto_impl! {
    /// An item of [Relative].
    #[derive(Clone, Debug, Hash)]
    pub enum RelativeItem {
        try_from{ },
        Length(Length=>Length),
        Ratio(Ratio=>Ratio),
    }
}

impl From<Length> for RelativeItem {
    fn from(value: Length) -> Self {
        RelativeItem::Length(value)
    }
}

impl From<Ratio> for RelativeItem {
    fn from(value: Ratio) -> Self {
        RelativeItem::Ratio(value)
    }
}

impl From<Length> for Relative {
    fn from(value: Length) -> Self {
        Relative(vec![RelativeItem::Length(value)].into())
    }
}

impl From<Ratio> for Relative {
    fn from(value: Ratio) -> Self {
        Relative(vec![RelativeItem::Ratio(value)].into())
    }
}

impl Default for RelativeItem {
    fn default() -> Self {
        RelativeItem::Ratio(Ratio::default())
    }
}

impl Display for Relative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self
            .0
            .iter()
            .map(|item| format!("{item:?}"))
            .collect::<Vec<_>>()
            .join(" + ");
        write!(f, "{repr}")
    }
}
