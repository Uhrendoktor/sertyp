use crate::{Symbol, String};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Symbol_<'a> {
    #[serde(borrow)]
    pub text: String<'a>
}

impl<'a> Into<Symbol<'a>> for Symbol_<'a> {
    fn into(self) -> Symbol<'a> {
        self.text.into()
    }
}

impl<'a> From<Symbol<'a>> for Symbol_<'a> {
    fn from(value: Symbol<'a>) -> Self {
        Symbol_ { text: value.0 }
    }
}

crate::impl_try_from_content!(Symbol<'a>);
crate::impl_into_content!(Symbol<'a>);