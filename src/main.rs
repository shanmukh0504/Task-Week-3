// use std::env;
// use crate::models::{depth_history::DepthHistory, runepool_history::RunePoolHistory, swaps_history::SwapHistory, earnings_history::EarningsHistory, pools_history::PoolHistory};
// use services::{fetch_depth::_fetch_and_store_data, fetch_runepool::_fetch_and_store_runepool_data, fetch_swaps::_fetch_and_store_swaps_data, fetch_earnings::_fetch_and_store_earnings_and_pools};

use api::{depth_history::depth_history_route, earnings::earnings_with_pools_route, runepool::runepool_history_route, swaps::swaps_history_route};
use dotenv::dotenv;
use std::error::Error;
use actix_web::{web, App, HttpServer};

mod api;
mod db;
mod models;
mod services;
mod data_fetcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db = db::connection::get_db().await?;
    // data_fetcher::_fetch_and_store_all_data(&db).await?;
    
    // let pool = env::var("POOL").expect("POOL must be set in .env");
    // let from_timestamp: i64 = env::var("START_TIME").expect("START_TIME must be set in .env").parse::<i64>().expect("Invalid START_TIME format");
    // let target_timestamp: i64 = env::var("END_TIME").expect("END_TIME must be set in .env").parse::<i64>().expect("Invalid END_TIME format");

    // let collection = db.database("historical_db").collection::<DepthHistory>("depth_history");
    // _fetch_and_store_data(pool.clone(), &collection, from_timestamp, target_timestamp).await?;
    // let collection = db.database("historical_db").collection::<RunePoolHistory>("runepool_history");
    // _fetch_and_store_runepool_data(&collection, from_timestamp, target_timestamp).await?;
    // let collection = db.database("historical_db").collection::<SwapHistory>("swaps_history");
    // _fetch_and_store_swaps_data(pool.clone(), &collection, from_timestamp, target_timestamp).await?;
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

    println!("Starting the server on port 3030...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(runepool_collection.clone()))
            .app_data(web::Data::new(depth_collection.clone()))
            .app_data(web::Data::new(swap_collection.clone()))
            .app_data(web::Data::new(earnings_collection.clone()))
            .app_data(web::Data::new(pools_collection.clone()))
            .route("/depth-history", web::get().to(depth_history_route))
            .route("/runepool-history", web::get().to(runepool_history_route))
            .route("/earnings", web::get().to(earnings_with_pools_route))
            .route("/swaps-history", web::get().to(swaps_history_route))
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await?;
    Ok(())
}