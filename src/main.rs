// use std::env;
// use crate::models::depth_history::DepthHistory;
// use services::fetch_depth::_fetch_and_store_data;
// use crate::models::runepool_history::RunePoolHistory;
// use services::fetch_runepool::_fetch_and_store_runepool_data;
// use crate::models::swaps_history::SwapHistory;
// use services::fetch_swaps::_fetch_and_store_swaps_data;
// use crate::models::earnings_history::EarningsHistory;
// use crate::models::pools_history::PoolHistory;
// use services::fetch_earnings::_fetch_and_store_earnings_and_pools;

use dotenv::dotenv;
use std::error::Error;
use warp::Filter;

mod api;
mod db;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db = db::connection::get_db().await?;
    
    // let pool = env::var("POOL").expect("POOL must be set in .env");
    // let from_timestamp: i64 = env::var("START_TIME").expect("START_TIME must be set in .env").parse::<i64>().expect("Invalid START_TIME format");
    // let target_timestamp: i64 = env::var("END_TIME").expect("END_TIME must be set in .env").parse::<i64>().expect("Invalid END_TIME format");

    // let collection = db.database("historical_db").collection::<DepthHistory>("depth_history");
    // _fetch_and_store_data(pool, &collection, from_timestamp, target_timestamp).await?;
    // let collection = db.database("historical_db").collection::<RunePoolHistory>("runepool_history");
    // _fetch_and_store_runepool_data(&collection, from_timestamp, target_timestamp).await?;
    // let collection = db.database("historical_db").collection::<SwapHistory>("swaps_history");
    // _fetch_and_store_swaps_data(pool, &collection, from_timestamp, target_timestamp).await?;
    // let earnings_collection = db.database("historical_db").collection::<EarningsHistory>("earnings_history");
    // let pools_collection = db.database("historical_db").collection::<PoolHistory>("pools_history");
    // _fetch_and_store_earnings_and_pools(&earnings_collection, &pools_collection, from_timestamp, target_timestamp).await?;


    // Routes

    let runepool_collection = db
        .database("historical_db")
        .collection::<models::runepool_history::RunePoolHistory>("runepool_history");
    let depth_collection = db
        .database("historical_db")
        .collection::<models::depth_history::DepthHistory>("depth_history");
    let swap_collection = db
        .database("historical_db")
        .collection::<models::swaps_history::SwapHistory>("swaps_history");
    let earnings_collection = db
        .database("historical_db")
        .collection::<models::earnings_history::EarningsHistory>("earnings_history");
    let pools_collection = db
        .database("historical_db")
        .collection::<models::pools_history::PoolHistory>("pools_history");

    let runepool_route = api::runepool::runepool_history_route(runepool_collection.clone());
    let depth_route = api::depth_history::depth_history_route(depth_collection.clone());
    let swaps_route = api::swaps::swaps_history_route(swap_collection.clone());
    let earnings_route = api::earnings::earnings_with_pools_route(
        earnings_collection.clone(),
        pools_collection.clone(),
    );
    let api_routes = warp::path("api").and(
        runepool_route
            .or(depth_route)
            .or(swaps_route)
            .or(earnings_route),
    );

    println!("Starting the server on port 3030...");
    warp::serve(api_routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}