use std::collections::HashMap;

use anyhow::Result;
pub struct CategoryService {}

static BASE: &str = "http://localhost:8080";

impl CategoryService {
    pub async fn add_category(name: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("name", name);

        println!("{} as service", name);
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
}
