use crate::types::{Item, r#type::TypeName};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(from = "&'a str", into = "&'a str")]
pub enum Function<'a> {
    Inline,
    Named(&'a str),
}

impl<'a> From<&'a str> for Function<'a> {
    fn from(value: &'a str) -> Self {
        if value == "(..) => .." {
            Function::Inline
        } else {
            Function::Named(value)
        }
    }
}

impl<'a> From<Function<'a>> for &'a str {
    fn from(value: Function<'a>) -> Self {
        match value {
            Function::Inline => "(..) => ..",
            Function::Named(name) => name,
        }
    }
}

impl<'a> Function<'a> {
    pub fn name(&self) -> Option<&'a str> {
        match self {
            Function::Inline => None,
            Function::Named(name) => name.rsplitn(1, ".").next(),
        }
    }

    pub fn full_name(&self) -> Option<&'a str> {
        match self {
            Function::Inline => None,
            Function::Named(name) => Some(name),
        }
    }

    pub fn ctx_name(&self) -> Option<&'a str> {
        match self {
            Function::Inline => None,
            Function::Named(name) => {
                let mut parts = name.rsplitn(1, ".");
                parts.next();
                parts.next()
            }
        }
    }
}

impl<'a> TryFrom<Item<'a>> for Function<'a> {
    type Error = std::string::String;

    fn try_from(value: Item<'a>) -> Result<Self, Self::Error> {
        match value {
            Item::Function(f) => Ok(f),
            _ => Err(format!("Invalid type for Function: {:?}", value)),
        }
    }
}

impl<'a> Into<Item<'a>> for Function<'a> {
    fn into(self) -> Item<'a> {
        Item::Function(self)
    }
}

impl<'a> TypeName for Function<'a> {
    fn name() -> &'static str {
        "function"
    }
}