use crate::services::category_service::CategoryService;
use anyhow::Result;
pub struct CategoryController {}

impl CategoryController {
    pub async fn add_category(name: String) -> Result<String> {
        println!("{} as controller", &name);
        CategoryService::add_category(&name).await?;
        Ok("カテゴリを追加しました！".to_string())
    }
}
