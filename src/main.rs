use api::{depth_history::depth_history_route, earnings::earnings_with_pools_route, runepool::runepool_history_route, swaps::swaps_history_route};
use dotenv::dotenv;
use std::error::Error;
use actix_web::{web, App, HttpServer};

mod api;
mod db;
mod models;
mod services;
mod cron_job;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db = db::connection::get_db().await?;

    let db_clone = db.clone();
    tokio::spawn(async move {
        if let Err(e) = cron_job::start_cron_job(db_clone).await {
            eprintln!("Error in cron job: {:?}", e);
        }
    });

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