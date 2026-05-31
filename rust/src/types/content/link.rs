use crate::{
    Dictionary, Label, TypedItem,
    types::{content::Content, string::String},
};

/// For more information visit the typst documentation: [link](https://typst.app/docs/reference/model/link/)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Link<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub dest: Option<LinkDestination<'a>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub body: Option<TypedItem<Box<Content<'a>>>>,
}

crate::impl_all!(Content<'a>::Link, Link<'a> {'a}, "link");

crate::auto_impl! {
    #[derive(Clone, Debug)]
    pub enum LinkDestination<'a> {
        try_from{},
        String(String=>String<'a>),
        Label(Label=>Label<'a>),
        // this is currently not supported by the serialization engine
        // Location(...),
        Dictionary(Dictionary=>Dictionary<'a>),
    }
}
