use std::ops::{Deref, DerefMut};

use crate::types::Item;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Styles<'a>(
    #[serde(borrow)]
    pub Box<Item<'a>>
);

impl<'a> Deref for Styles<'a> {
    type Target = Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Styles<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

crate::impl_all!(Styles<'a>, "styles");