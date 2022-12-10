use super::super::poise_data::*;
use chrono::{Utc, Datelike};
use serde_json::{json, Value};

/// get total amount of this month
#[poise::command(slash_command, prefix_command)]
pub async fn get_this_month( ctx: Context<'_>) -> Result<(), Error> {
    let total_amount = get_total_amount_this_month().await?.parse::<i64>().unwrap();
    let today = Utc::now().day() as i64;
    let amount_per_day = total_amount as f64 / today as f64;
    let estimate_total = amount_per_day * 30.0;
    let comment = match estimate_total as i64{
        0..=100000 => "安上がりな人生！",
        100001..=120000 => "いい感じ！",
        120001..=150000 => "まあまあこんぐらいはしゃーない。",
        150001.. => "ほんまにお金なくなるで。",
        _ => "Bag Occured."
    };
    
    ctx.say(
        format!("
            今月の総支払額は{}円です！\n1日当たり{}円使っています！\nこのまま行くと月末の支払い額は{}になります!\n{}",
            total_amount,
            amount_per_day,
            estimate_total,
            comment,
        )
    ).await?;


    Ok(())
}

async fn get_total_amount_this_month() -> Result<String, Error> {
    let url = format!("http://localhost:8000/log/this_month");
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .send()
        .await?;

    let body = resp.text().await? ;
    Ok(body)
}
