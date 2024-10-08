use std::env;
use futures::StreamExt;
use mongodb::{Collection, bson::doc};
use crate::models::{depth_history::DepthHistory, runepool_history::RunePoolHistory, swaps_history::SwapHistory, earnings_history::EarningsHistory, pools_history::PoolHistory};
use crate::services::{fetch_depth::_fetch_and_store_data, fetch_runepool::_fetch_and_store_runepool_data, fetch_swaps::_fetch_and_store_swaps_data, fetch_earnings::_fetch_and_store_earnings_and_pools};
use chrono::Utc;

pub async fn _fetch_and_store_all_data(db: &mongodb::Client) -> Result<(), Box<dyn std::error::Error>> {
    let pool = env::var("POOL").expect("POOL must be set in .env");
    let target_timestamp = Utc::now().timestamp();
    
    let depth_collection = db.database("historical_db").collection::<DepthHistory>("depth_history");
    let mut from_timestamp = _get_last_end_time::<DepthHistory>(&depth_collection).await?;
    _fetch_and_store_data(pool.clone(), &depth_collection, from_timestamp, target_timestamp).await?;

    let rune_collection = db.database("historical_db").collection::<RunePoolHistory>("runepool_history");
    from_timestamp = _get_last_end_time::<RunePoolHistory>(&rune_collection).await?;
    _fetch_and_store_runepool_data(&rune_collection, from_timestamp, target_timestamp).await?;

    let swap_collection = db.database("historical_db").collection::<SwapHistory>("swaps_history");
    from_timestamp = _get_last_end_time::<SwapHistory>(&swap_collection).await?;
    _fetch_and_store_swaps_data(pool.clone(), &swap_collection, from_timestamp, target_timestamp).await?;

    let earnings_collection = db.database("historical_db").collection::<EarningsHistory>("earnings_history");
    let pools_collection = db.database("historical_db").collection::<PoolHistory>("pools_history");
    from_timestamp = _get_last_end_time::<EarningsHistory>(&earnings_collection).await?;
    _fetch_and_store_earnings_and_pools(&earnings_collection, &pools_collection, from_timestamp, target_timestamp).await?;

    Ok(())
}

async fn _get_last_end_time<T>(collection: &Collection<T>) -> Result<i64, Box<dyn std::error::Error>>
where
T: serde::de::DeserializeOwned + serde::Serialize + Unpin + Send + Sync,
{
    let filter = doc! {};

    let mut cursor = collection.find(filter).await?;

    let mut documents: Vec<T> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => documents.push(doc),
            Err(err) => {
                eprintln!("Error fetching document: {:?}", err);
                continue;
            }
        }
    }

    documents.sort_by_key(|doc| {
        let bson_doc = bson::to_document(doc).unwrap_or_else(|_| doc! {});
        -bson_doc.get_i64("endTime").unwrap_or(0)
    });

    if let Some(doc) = documents.first() {
        let bson_doc = bson::to_document(doc).unwrap_or_else(|_| doc! {});
        let end_time = bson_doc.get_i64("endTime").unwrap_or(0);
        Ok(end_time)
    } else {
        Ok(0)
    }
}
