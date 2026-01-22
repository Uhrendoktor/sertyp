use crate::{Symbol, String, types::array::Pair};

crate::auto_impl!{
    #[derive(Clone, Debug)]
    pub enum Delim<'a> {
        #[try_from]
        Double(Pair<String<'a>>),
        Single(String<'a>),
        Symbol(Symbol<'a>),
    }
}

impl<'a> Default for Delim<'a> {
    fn default() -> Self {
        Delim::Double(Pair(String::from("("), String::from(")")))
    }
}