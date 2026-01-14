use crate::Symbol;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Symbol_<'a> {
    #[serde(borrow)]
    pub text: Symbol<'a>
}

impl<'a> Into<Symbol<'a>> for Symbol_<'a> {
    fn into(self) -> Symbol<'a> {
        self.text
    }
}

impl<'a> From<Symbol<'a>> for Symbol_<'a> {
    fn from(value: Symbol<'a>) -> Self {
        Symbol_ { text: value }
    }
}

crate::impl_try_from_content!(Symbol<'a>);
crate::impl_into_content!(Symbol<'a>);