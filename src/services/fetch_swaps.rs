use reqwest::get;
use serde_json::Value;
use mongodb::Collection;
use crate::models::swaps_history::SwapHistory;
use std::error::Error;

pub async fn _fetch_and_store_swaps_data(
    pool: String,
    collection: &Collection<SwapHistory>,
    from_timestamp: i64,
    target_timestamp: i64,
) -> Result<(), Box<dyn Error>> {
    let mut current_timestamp = from_timestamp;

    println!("Fetching swaps history");

    while current_timestamp <= target_timestamp {
        let api_url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&pool={}&count=400&from={}",
            pool, current_timestamp
        );

        let (fetched_data, latest_end_time) = _fetch_swap_data(&api_url, &pool).await?;

        crate::db::insert_swap::_insert_swap_history(collection, fetched_data).await?;

        println!("Data inserted successfully for timestamp {}", current_timestamp);

        current_timestamp = latest_end_time;
    }

    Ok(())
}

pub async fn _fetch_swap_data(api_url: &str, pool: &str) -> Result<(Vec<SwapHistory>, i64), Box<dyn Error>> {
    let response = get(api_url).await?.text().await?;

    let json: Value = serde_json::from_str(&response)?;

    let intervals = json["intervals"].as_array().ok_or("Invalid intervals format")?;
    let meta_end_time = json["meta"]["endTime"]
        .as_str()
        .ok_or("Missing endTime in meta")?
        .parse::<i64>()?;

    let data: Vec<SwapHistory> = intervals.iter()
        .filter_map(|interval| {
            let swap_history = SwapHistory {
                id: None,
                pool: pool.to_string(),
                average_slip: interval["averageSlip"].as_str()?.parse::<f64>().ok()?,
                end_time: interval["endTime"].as_str()?.parse::<i64>().ok()?,
                rune_price_usd: interval["runePriceUSD"].as_str()?.parse::<f64>().ok()?,
                start_time: interval["startTime"].as_str()?.parse::<i64>().ok()?,
                synth_mint_average_slip: interval["synthMintAverageSlip"].as_str()?.parse::<f64>().ok()?,
                synth_mint_count: interval["synthMintCount"].as_str()?.parse::<i64>().ok()?,
                synth_mint_fees: interval["synthMintFees"].as_str()?.parse::<i64>().ok()?,
                synth_mint_volume: interval["synthMintVolume"].as_str()?.parse::<i64>().ok()?,
                synth_mint_volume_usd: interval["synthMintVolumeUSD"].as_str()?.parse::<f64>().ok()?,
                synth_redeem_average_slip: interval["synthRedeemAverageSlip"].as_str()?.parse::<f64>().ok()?,
                synth_redeem_count: interval["synthRedeemCount"].as_str()?.parse::<i64>().ok()?,
                synth_redeem_fees: interval["synthRedeemFees"].as_str()?.parse::<i64>().ok()?,
                synth_redeem_volume: interval["synthRedeemVolume"].as_str()?.parse::<i64>().ok()?,
                synth_redeem_volume_usd: interval["synthRedeemVolumeUSD"].as_str()?.parse::<f64>().ok()?,
                to_asset_average_slip: interval["toAssetAverageSlip"].as_str()?.parse::<f64>().ok()?,
                to_asset_count: interval["toAssetCount"].as_str()?.parse::<i64>().ok()?,
                to_asset_fees: interval["toAssetFees"].as_str()?.parse::<i64>().ok()?,
                to_asset_volume: interval["toAssetVolume"].as_str()?.parse::<i64>().ok()?,
                to_asset_volume_usd: interval["toAssetVolumeUSD"].as_str()?.parse::<f64>().ok()?,
                to_rune_average_slip: interval["toRuneAverageSlip"].as_str()?.parse::<f64>().ok()?,
                to_rune_count: interval["toRuneCount"].as_str()?.parse::<i64>().ok()?,
                to_rune_fees: interval["toRuneFees"].as_str()?.parse::<i64>().ok()?,
                to_rune_volume: interval["toRuneVolume"].as_str()?.parse::<i64>().ok()?,
                to_rune_volume_usd: interval["toRuneVolumeUSD"].as_str()?.parse::<f64>().ok()?,
                total_count: interval["totalCount"].as_str()?.parse::<i64>().ok()?,
                total_fees: interval["totalFees"].as_str()?.parse::<i64>().ok()?,
                total_volume: interval["totalVolume"].as_str()?.parse::<i64>().ok()?,
                total_volume_usd: interval["totalVolumeUSD"].as_str()?.parse::<f64>().ok()?,
            };
            Some(swap_history)
        })
        .collect();
    Ok((data, meta_end_time))
}
