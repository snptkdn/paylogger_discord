use std::collections::HashMap;

use crate::constraits::BASE;
use anyhow::Result;
use chrono::Local;

pub struct PurchaseLogService {}

impl PurchaseLogService {
    pub async fn add_log_purchase(price: i64, category: u64, date: Option<String>) -> Result<()> {
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("price", price.to_string());
        params.insert("category", category.to_string());
        if let Some(date) = date {
            params.insert("date", date.to_string());
        } else {
            let utc_date = Local::now().naive_local().date().format("%Y%m%d");
            params.insert("date", utc_date.to_string());
        };

        let res = client
            .post(format!("{}/purchase_log", BASE))
            .form(&params)
            .send()
            .await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
