use crate::models::{
    depth_history::DepthHistory, earnings_history::EarningsHistory, pools_history::PoolHistory,
    runepool_history::RunePoolHistory, swaps_history::SwapHistory,
};
use crate::services::{
    fetch_depth::_fetch_and_store_data, fetch_earnings::_fetch_and_store_earnings_and_pools,
    fetch_runepool::_fetch_and_store_runepool_data, fetch_swaps::_fetch_and_store_swaps_data,
};
use chrono::{Duration as ChronoDuration, Utc};
use cron::Schedule;
use std::env;
use std::error::Error;
use std::str::FromStr;
use tokio::time::sleep;
use tokio::time::Duration;

pub async fn start_cron_job(db: mongodb::Client) -> Result<(), Box<dyn Error>> {
    let pool = env::var("POOL").expect("POOL must be set in .env");

    let schedule = Schedule::from_str("0 * * * *")?;

    loop {
        let now = Utc::now();
        let next_run = schedule.upcoming(Utc).next().unwrap();

        if now < next_run {
            sleep(Duration::from_secs((next_run - now).num_seconds() as u64)).await;
        }

        let target_timestamp = Utc::now().timestamp();
        let from_timestamp = (Utc::now() - ChronoDuration::hours(1)).timestamp();

        let depth_collection = db
            .database("historical_db")
            .collection::<DepthHistory>("depth_history");
        _fetch_and_store_data(pool.clone(), &depth_collection, from_timestamp, target_timestamp).await?;

        let collection = db
            .database("historical_db")
            .collection::<RunePoolHistory>("runepool_history");
        _fetch_and_store_runepool_data(&collection, from_timestamp, target_timestamp).await?;
        let collection = db
            .database("historical_db")
            .collection::<SwapHistory>("swaps_history");
        _fetch_and_store_swaps_data(pool.clone(), &collection, from_timestamp, target_timestamp).await?;
        let earnings_collection = db
            .database("historical_db")
            .collection::<EarningsHistory>("earnings_history");
        let pools_collection = db
            .database("historical_db")
            .collection::<PoolHistory>("pools_history");
        _fetch_and_store_earnings_and_pools(
            &earnings_collection,
            &pools_collection,
            from_timestamp,
            target_timestamp,
        )
        .await?;
    }
}
