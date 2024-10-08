use mongodb::Collection;
use reqwest::get;
use serde_json::Value;
use crate::models::runepool_history::RunePoolHistory;
use std::error::Error;

pub async fn _fetch_and_store_runepool_data(
    collection: &Collection<RunePoolHistory>,
    from_timestamp: i64,
    target_timestamp: i64,
) -> Result<(), Box<dyn Error>> {
    let mut current_timestamp = from_timestamp;

    println!("Fetching runepool history");

    while current_timestamp <= target_timestamp {
        let api_url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count=400&from={}",
            current_timestamp
        );

        let (fetched_data, latest_end_time) = _fetch_runepool_data(&api_url).await?;

        crate::db::insert_runepool::_insert_runepool_history(collection, fetched_data).await?;

        println!("Data inserted successfully for timestamp {}", current_timestamp);

        current_timestamp = latest_end_time;

    }

    Ok(())
}

pub async fn _fetch_runepool_data(url: &str) -> Result<(Vec<RunePoolHistory>, i64), Box<dyn Error>> {
    let response = get(url).await?.text().await?;

    let json: Value = serde_json::from_str(&response)?;

    let intervals = json["intervals"].as_array().ok_or("Invalid intervals format")?;
    
    let meta_end_time = json["meta"]["endTime"]
        .as_str()
        .ok_or("Missing endTime in meta")?
        .parse::<i64>()
        .map_err(|_| "Invalid endTime format")?;

    let data: Vec<RunePoolHistory> = intervals.iter()
        .filter_map(|interval| {
            let runepool_history = RunePoolHistory {
                id: None,
                count: interval["count"].as_str()?.parse::<i64>().ok()?,
                start_time: interval["startTime"].as_str()?.parse::<i64>().ok()?,
                end_time: interval["endTime"].as_str()?.parse::<i64>().ok()?,
                units: interval["units"].as_str()?.parse::<i64>().ok()?,
            };
            Some(runepool_history)
        })
        .collect();

    Ok((data, meta_end_time))
}