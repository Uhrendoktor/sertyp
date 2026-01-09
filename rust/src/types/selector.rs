use crate::types::r#type::TypeName;

// For future use
#[allow(dead_code)]
pub struct Selector;

impl<'a> TypeName for Selector {
    fn name() -> &'static str {
        "selector"
    }
}