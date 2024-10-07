use warp::Filter;
use mongodb::Collection;
use crate::models::{earnings_history::EarningsHistory, pools_history::PoolHistory};
use mongodb::bson::{doc, to_document};
use futures::stream::StreamExt;

pub fn earnings_with_pools_route(
    earnings_collection: Collection<EarningsHistory>,
    pools_collection: Collection<PoolHistory>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("earnings-with-pools")
        .and(warp::get())
        .and_then({
            let earnings_collection = earnings_collection.clone();
            let pools_collection = pools_collection.clone();

            move || {
                let earnings_collection = earnings_collection.clone();
                let pools_collection = pools_collection.clone();

                async move {
                    let mut earnings_cursor = match earnings_collection.find(doc! {}).await {
                        Ok(cursor) => cursor,
                        Err(e) => {
                            eprintln!("Error fetching earnings data: {:?}", e);
                            return Ok::<_, warp::Rejection>(warp::reply::with_status(
                                warp::reply::json(&"Error fetching earnings data"),
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ));
                        }
                    };

                    let mut earnings_with_pools = Vec::new();

                    while let Some(earnings_result) = earnings_cursor.next().await {
                        match earnings_result {
                            Ok(earnings) => {
                                let mut pools_cursor = match pools_collection
                                    .find(doc! { "earnings_id": earnings.id.unwrap() })
                                    .await
                                {
                                    Ok(cursor) => cursor,
                                    Err(e) => {
                                        eprintln!("Error fetching pools data: {:?}", e);
                                        return Ok::<_, warp::Rejection>(warp::reply::with_status(
                                            warp::reply::json(&"Error fetching pools data"),
                                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        ));
                                    }
                                };

                                let mut pools = Vec::new();
                                while let Some(pool_result) = pools_cursor.next().await {
                                    match pool_result {
                                        Ok(pool) => {
                                            let pool_doc = to_document(&pool).unwrap();
                                            pools.push(pool_doc);
                                        }
                                        Err(e) => {
                                            eprintln!("Error processing pool data: {:?}", e);
                                            return Ok::<_, warp::Rejection>(warp::reply::with_status(
                                                warp::reply::json(&"Error processing pool data"),
                                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                            ));
                                        }
                                    }
                                }

                                let mut earnings_doc = to_document(&earnings).unwrap();
                                earnings_doc.remove("_id");

                                earnings_doc.insert("pools", pools);

                                earnings_with_pools.push(earnings_doc);
                            }
                            Err(e) => {
                                eprintln!("Error processing earnings data: {:?}", e);
                                return Ok::<_, warp::Rejection>(warp::reply::with_status(
                                    warp::reply::json(&"Error processing earnings data"),
                                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        }
                    }

                    if earnings_with_pools.is_empty() {
                        Ok::<_, warp::Rejection>(warp::reply::with_status(
                            warp::reply::json(&"No data found"),
                            warp::http::StatusCode::NOT_FOUND,
                        ))
                    } else {
                        Ok::<_, warp::Rejection>(warp::reply::with_status(
                            warp::reply::json(&earnings_with_pools),
                            warp::http::StatusCode::OK,
                        ))
                    }
                }
            }
        })
}