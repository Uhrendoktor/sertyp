use crate::types::string::String;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(from = "String", into = "String")]
pub enum Function<'a> {
    Inline,
    #[serde(borrow)]
    Named(String<'a>),
}

crate::impl_all!(Function<'a>, "function");

impl<'a> From<String<'a>> for Function<'a> {
    fn from(value: String<'a>) -> Self {
        if &*value == "(..) => .." {
            Function::Inline
        } else {
            Function::Named(value)
        }
    }
}

impl<'a> Into<String<'a>> for Function<'a> {
    fn into(self) -> String<'a> {
        match self {
            Function::Inline => "(..) => ..".into(),
            Function::Named(name) => name,
        }
    }
}

impl<'a> Function<'a> {
    pub fn name(&'a self) -> Option<String<'a>> {
        match self {
            Function::Inline => None,
            Function::Named(name) => (&*name).rsplitn(1, ".").next().map(|s| s.into()),
        }
    }

    pub fn full_name(&'a self) -> Option<String<'a>> {
        match self {
            Function::Inline => None,
            Function::Named(name) => Some((&**name).into()),
        }
    }

    pub fn ctx_name(&'a self) -> Option<String<'a>> {
        match self {
            Function::Inline => None,
            Function::Named(name) => {
                let mut parts = name.rsplitn(1, ".");
                parts.next();
                parts.next().map(|s| s.into())
            }
        }
    }
}