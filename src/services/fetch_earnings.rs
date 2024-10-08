use crate::models::earnings_history::EarningsHistory;
use crate::models::pools_history::PoolHistory;
use bson::oid::ObjectId;
use mongodb::Collection;
use reqwest::get;
use serde_json::Value;
use std::error::Error;

pub async fn _fetch_and_store_earnings_and_pools(
    earnings_collection: &Collection<EarningsHistory>,
    pools_collection: &Collection<PoolHistory>,
    from_timestamp: i64,
    target_timestamp: i64,
) -> Result<(), Box<dyn Error>> {
    let mut current_timestamp = from_timestamp;
    
    println!("Fetching earnings history");

    while current_timestamp <= target_timestamp {
        let api_url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval=hour&count=400&from={}",
            current_timestamp
        );

        let (earnings_data, pools_data, latest_end_time) = _fetch_earnings_data(&api_url).await?;

        let inserted_earnings_ids =
            crate::db::insert_earnings::_insert_earnings(earnings_collection, earnings_data).await?;

        for (index, pools) in pools_data.into_iter().enumerate() {
            if let Some(earnings_id) = inserted_earnings_ids.get(index) {
                let updated_pools: Vec<PoolHistory> = pools
                    .into_iter()
                    .map(|mut pool| {
                        pool.earnings_id = *earnings_id;
                        pool
                    })
                    .collect();

                let earnings_id_vec = vec![*earnings_id];
                crate::db::insert_pools::_insert_pools(pools_collection, updated_pools, &earnings_id_vec).await?;
            }
        }

        println!("Data inserted successfully for timestamp {}", current_timestamp);

        current_timestamp = latest_end_time;
    }

    Ok(())
}

pub async fn _fetch_earnings_data(
    url: &str,
) -> Result<(Vec<EarningsHistory>, Vec<Vec<PoolHistory>>, i64), Box<dyn Error>> {
    let response = get(url).await?.text().await?;

    let json: Value = serde_json::from_str(&response)?;

    let intervals = json["intervals"]
        .as_array()
        .ok_or("Invalid intervals format")?;

    let meta_end_time = json["meta"]["endTime"]
        .as_str()
        .ok_or("Missing endTime in meta")?
        .parse::<i64>()?;

    let mut earnings_data: Vec<EarningsHistory> = Vec::new();
    let mut pools_data: Vec<Vec<PoolHistory>> = Vec::new();

    for interval in intervals {
        let earnings_history = EarningsHistory {
            id: None,
            avg_node_count: interval["avgNodeCount"]
                .as_str()
                .ok_or("Missing avgNodeCount")?
                .parse::<f64>()?,
            block_rewards: interval["blockRewards"]
                .as_str()
                .ok_or("Missing blockRewards")?
                .parse::<i64>()?,
            bonding_earnings: interval["bondingEarnings"]
                .as_str()
                .ok_or("Missing bondingEarnings")?
                .parse::<i64>()?,
            earnings: interval["earnings"]
                .as_str()
                .ok_or("Missing earnings")?
                .parse::<i64>()?,
            end_time: interval["endTime"]
                .as_str()
                .ok_or("Missing endTime")?
                .parse::<i64>()?,
            liquidity_earnings: interval["liquidityEarnings"]
                .as_str()
                .ok_or("Missing liquidityEarnings")?
                .parse::<i64>()?,
            liquidity_fees: interval["liquidityFees"]
                .as_str()
                .ok_or("Missing liquidityFees")?
                .parse::<i64>()?,
            rune_price_usd: interval["runePriceUSD"]
                .as_str()
                .ok_or("Missing runePriceUSD")?
                .parse::<f64>()?,
            start_time: interval["startTime"]
                .as_str()
                .ok_or("Missing startTime")?
                .parse::<i64>()?,
        };

        earnings_data.push(earnings_history);

        let pools = interval["pools"].as_array();
        if let Some(pools_array) = pools {
            let pool_history_data: Vec<PoolHistory> = pools_array
                .iter()
                .filter_map(|pool| {
                    Some(PoolHistory {
                        id: None,
                        earnings_id: ObjectId::new(),
                        asset_liquidity_fees: pool["assetLiquidityFees"]
                            .as_str()?
                            .parse::<i64>()
                            .ok()?,
                        earnings: pool["earnings"].as_str()?.parse::<i64>().ok()?,
                        pool: pool["pool"].to_string(),
                        rewards: pool["rewards"].as_str()?.parse::<i64>().ok()?,
                        rune_liquidity_fees: pool["runeLiquidityFees"]
                            .as_str()?
                            .parse::<i64>()
                            .ok()?,
                        saver_earning: pool["saverEarning"].as_str()?.parse::<i64>().ok()?,
                        total_liquidity_fees_rune: pool["totalLiquidityFeesRune"]
                            .as_str()?
                            .parse::<i64>()
                            .ok()?,
                    })
                })
                .collect();

            pools_data.push(pool_history_data);
        }
    }

    Ok((earnings_data, pools_data, meta_end_time))
}
