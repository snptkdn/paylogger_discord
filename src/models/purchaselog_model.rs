use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurchaseLogModel {
    #[serde(rename = "ID")]
    pub id: i64,
    pub price: String,
    pub category_id: u64,
    pub date: String,
}
