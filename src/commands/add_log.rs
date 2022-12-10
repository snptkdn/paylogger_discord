use super::super::poise_data::*;
use chrono::Utc;
use serde_json::{json, Value};


/// add weight of day
#[poise::command(slash_command, prefix_command)]
pub async fn add_log(
    ctx: Context<'_>,
    #[description = "price(yen)"] price: i64,
    #[description = "category"] category: String,
) -> Result<(), Error> {
    let date = Utc::now().date_naive();
    let resp = get_category_id(category).await;

    let id = match resp {
        Ok(resp) => Some(resp[0].get("id").unwrap().clone()),
        Err(_) => None
    };

    if let Some(id) = id {
        let body = json!({
            "price": price,
            "category": id,
            "buy_date" : date
        });

        println!("{:?}", body);

        let client = reqwest::Client::new();
        let url = "http://localhost:8000/log";
        client.post(url)
            .json(&body)
            .send()
            .await?;

        ctx.say("登録しました！無駄遣いしないでね！").await?;
    } else {
        ctx.say("そのカテゴリは登録されていません！").await?;

        let resp = get_all_category().await?;
        ctx.say(format!("現在登録されているカテゴリは以下です。{}", resp)).await?;
    }

    


    Ok(())
}

async fn get_category_id(name: String) -> Result<Value, Error> {
    let url = format!("http://localhost:8000/category?name={}", name);
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .send()
        .await?;

    let body = resp.text().await? ;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    Ok(json)
}

async fn get_all_category() -> Result<Value, Error> {
    let url = format!("http://localhost:8000/category");
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .send()
        .await?;

    let body = resp.text().await? ;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    Ok(json)
}
