#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Space;

crate::impl_all_content!(Space, "space");