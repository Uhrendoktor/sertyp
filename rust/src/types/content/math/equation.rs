use crate::{Alignment, AutoOr, Boolean, Content, Function, Or, RBox, String, TypedItem};

/// For more information visit the typst documentation: [math.equation](https://typst.app/docs/reference/math/equation/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, Hash)]
pub struct Equation<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<TypedItem<Boolean>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub numbering: Option<Or<String<'a>, Function<'a>>>,
    #[serde(rename = "number-align", skip_serializing_if = "Option::is_none")]
    pub number_align: Option<TypedItem<Alignment>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub supplement: Option<AutoOr<Or<RBox<Content<'a>>, Function<'a>>>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub alt: Option<TypedItem<String<'a>>>,
    pub body: TypedItem<RBox<Content<'a>>>,
}

crate::impl_all!(Content<'a>::MathEquation, Equation<'a>{'a}, "math.equation");

impl<'a> Equation<'a> {
    pub fn new(body: Content<'a>) -> Self {
        Equation {
            body: TypedItem::new(RBox::new(body)),
            ..Equation::default()
        }
    }
}
