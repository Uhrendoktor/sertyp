use std::fmt::Display;

use crate::types::array::Array;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Relative<'a>(
    #[serde(borrow)]
    pub Array<'a>
);

crate::impl_all!(Relative<'a>, "relative");

impl Display for Relative<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // join with +
        let s = self.0.iter()
            .map(|item| format!("{:?}", item))
            .collect::<Vec<_>>()
            .join(" + ");
        write!(f, "{}", s)    
    }
}