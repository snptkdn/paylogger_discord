use super::super::poise_data::*;
use chrono::{Utc, Datelike};
use serde_json::{json, Value};

/// add category
#[poise::command(slash_command, prefix_command)]
pub async fn add_category(
    ctx: Context<'_>,
    #[description = "category's name"] name: String,
 ) -> Result<(), Error> {
    
    let body = json!({
        "name": name
    });

    let client = reqwest::Client::new();
    let url = "http://localhost:8000/category";
    client.post(url)
        .json(&body)
        .send()
        .await?;

    ctx.say("カテゴリを登録しました！").await?;

    Ok(())
}
