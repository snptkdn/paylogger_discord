use crate::services::purchaselog_service::PurchaseLogService;
use anyhow::Result;
pub struct PurchaseLogController {}

impl PurchaseLogController {
    pub async fn add_log_purchase(
        price: i64,
        category: u64,
        date: Option<String>,
    ) -> Result<String> {
        PurchaseLogService::add_log_purchase(price, category, date).await?;
        Ok("ログを追加しました！".to_string())
    }
}
