use std::collections::HashMap;

use crate::constraits::BASE;
use crate::models::category_model::CategoryModel;
use anyhow::Result;
pub struct CategoryService {}

impl CategoryService {
    pub async fn add_category(name: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("name", name.replace("\\", "").replace("\"", ""));

        let res = client
            .post(format!("{}/category", BASE))
            .form(&params)
            .send()
            .await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get_categories() -> Result<Vec<CategoryModel>> {
        let client = reqwest::Client::new();

        let res = client.get(format!("{}/category", BASE)).send().await?;

        Ok(res.json().await?)
    }
}
