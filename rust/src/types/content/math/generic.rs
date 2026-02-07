use crate::{String, Symbol, types::array::Pair};

crate::auto_impl! {
    /// Delimiter type. Used for example in `math.cases`. For more information visit the typst documentation: [delim](https://typst.app/docs/reference/math/cases/#parameters-delim)
    #[derive(Clone, Debug, Hash)]
    pub enum Delim<'a> {
        try_from {
            Double(Pair<String<'a>>),
        },
        Single(String=>String<'a>),
        Symbol(Symbol=>Symbol<'a>),
    }
}

impl<'a> Default for Delim<'a> {
    fn default() -> Self {
        Delim::Double(Pair(String::from("("), String::from(")")))
    }
}
