use crate::models::category_model::CategoryModel;
use crate::services::category_service::CategoryService;
use anyhow::Result;
pub struct CategoryController {}

impl CategoryController {
    pub async fn add_category(name: String) -> Result<String> {
        CategoryService::add_category(&name).await?;
        Ok("カテゴリを追加しました！".to_string())
    }

    pub async fn get_categories() -> Result<Vec<CategoryModel>> {
        let res = CategoryService::get_categories().await?;
        Ok(res)
    }
}
