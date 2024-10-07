use reqwest::get;
use serde_json::Value;
use crate::models::depth_history::DepthHistory;
use mongodb::Collection;
use std::error::Error;

pub async fn _fetch_and_store_data(
    pool: String,
    collection: &Collection<DepthHistory>,
    from_timestamp: i64,
    target_timestamp: i64,
) -> Result<(), Box<dyn Error>> {
    let mut current_timestamp = from_timestamp;

    while current_timestamp <= target_timestamp {
        let api_url = format!(
            "https://midgard.ninerealms.com/v2/history/depths/{}?interval=hour&count=400&from={}",
            pool, current_timestamp
        );

        let (fetched_data, latest_end_time) = _fetch_data(&api_url, &pool).await?;

        crate::db::insert_depth::_insert_depth_history(collection, fetched_data).await?;

        println!("Data inserted successfully for timestamp {}", current_timestamp);

        current_timestamp = latest_end_time;
    }

    Ok(())
}

pub async fn _fetch_data(url: &str, pool: &str) -> Result<(Vec<DepthHistory>, i64), Box<dyn Error>> {
    let response = get(url).await?.text().await?;

    let json: Value = serde_json::from_str(&response)?;

    let intervals = json["intervals"].as_array().ok_or("Invalid intervals format")?;

    let meta_end_time = json["meta"]["endTime"]
        .as_str()
        .ok_or("Missing endTime in meta")?
        .parse::<i64>()?;

    let data: Vec<DepthHistory> = intervals.iter()
        .filter_map(|interval| {
            let depth_history = DepthHistory {
                id: None,
                pool: pool.to_string(),
                start_time: interval["startTime"].as_str()?.parse::<i64>().ok()?,
                end_time: interval["endTime"].as_str()?.parse::<i64>().ok()?,
                asset_depth: interval["assetDepth"].as_str()?.parse::<i64>().ok()?,
                asset_price: interval["assetPrice"].as_str()?.parse::<f64>().ok()?,
                asset_price_usd: interval["assetPriceUSD"].as_str()?.parse::<f64>().ok()?,
                liquidity_units: interval["liquidityUnits"].as_str()?.parse::<i64>().ok()?,
                members_count: interval["membersCount"].as_str()?.parse::<i64>().ok()?,
                rune_depth: interval["runeDepth"].as_str()?.parse::<i64>().ok()?,
                synth_supply: interval["synthSupply"].as_str()?.parse::<i64>().ok()?,
                synth_units: interval["synthUnits"].as_str()?.parse::<i64>().ok()?,
                units: interval["units"].as_str()?.parse::<i64>().ok()?,
                luvi: interval["luvi"].as_str()?.parse::<f64>().ok()?,
            };
            println!("Parsed DepthHistory: {:?}", depth_history); // Add this line
            Some(depth_history)
        })
        .collect();

    Ok((data, meta_end_time))
}
