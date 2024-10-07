use warp::Filter;
use mongodb::Collection;
use crate::models::{earnings_history::EarningsHistory, pools_history::PoolHistory};
use mongodb::bson::{doc, to_document};
use futures::stream::StreamExt;
use serde::Deserialize;
use warp::reply::{json, with_status};
use warp::http::StatusCode;

#[derive(Debug, Deserialize)]
pub struct EarningsWithPoolsQueryParams {
    start_time: Option<i64>,
    end_time: Option<i64>,
    summary: Option<bool>,
    page: Option<u32>,
    limit: Option<u32>,
    sort_by: Option<String>,
    order: Option<String>,
}

pub fn earnings_with_pools_route(
    earnings_collection: Collection<EarningsHistory>,
    pools_collection: Collection<PoolHistory>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("earnings-with-pools")
        .and(warp::get())
        .and(warp::query::<EarningsWithPoolsQueryParams>())
        .and_then({
            let earnings_collection = earnings_collection.clone();
            let pools_collection = pools_collection.clone();

            move |params: EarningsWithPoolsQueryParams| {
                let earnings_collection = earnings_collection.clone();
                let pools_collection = pools_collection.clone();

                async move {
                    let mut filter_conditions = Vec::new();

                    if let (Some(start), Some(end)) = (params.start_time, params.end_time) {
                        if start >= end {
                            return Ok::<_, warp::Rejection>(with_status(
                                json(&"start_time must be less than end_time"),
                                StatusCode::BAD_REQUEST,
                            ));
                        }
                        filter_conditions.push(doc! { "startTime": { "$gte": start } });
                        filter_conditions.push(doc! { "endTime": { "$lte": end } });
                    } else if let Some(start) = params.start_time {
                        filter_conditions.push(doc! { "startTime": { "$gte": start } });
                    } else if let Some(end) = params.end_time {
                        filter_conditions.push(doc! { "endTime": { "$lte": end } });
                    }
                    
                    let filter_doc = if filter_conditions.is_empty() {
                        doc! {}
                    } else {
                        doc! { "$and": filter_conditions }
                    };

                    let limit = params.limit.unwrap_or(10).clamp(1, 100) as i64;
                    let page = params.page.unwrap_or(1).max(1);
                    let skip = (page - 1) * limit as u32;

                    let sort_doc = if let Some(sort_by) = params.sort_by {
                        let sort_order = match params.order.as_deref() {
                            Some("asc") => 1,
                            Some("desc") | _ => -1,
                        };
                        doc! { sort_by: sort_order }
                    } else {
                        doc! { "startTime": -1 }
                    };

                    if params.summary.unwrap_or(false) {
                        let mut earnings_cursor = match earnings_collection.find(filter_doc.clone())
                            .sort(sort_doc.clone())
                            .skip(skip as u64)
                            .limit(limit)
                            .await {
                            Ok(cursor) => cursor,
                            Err(e) => {
                                eprintln!("Error fetching earnings data: {:?}", e);
                                return Ok::<_, warp::Rejection>(with_status(
                                    json(&"Error fetching earnings data"),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        };

                        let mut earnings_with_pools = Vec::new();

                        while let Some(earnings_result) = earnings_cursor.next().await {
                            match earnings_result {
                                Ok(earnings) => {
                                    let mut pools_cursor = match pools_collection
                                        .find(doc! { "earnings_id": earnings.id.unwrap() })
                                        .sort(sort_doc.clone()) 
                                        .limit(10) 
                                        .await
                                    {
                                        Ok(cursor) => cursor,
                                        Err(e) => {
                                            eprintln!("Error fetching pools data: {:?}", e);
                                            return Ok::<_, warp::Rejection>(with_status(
                                                json(&"Error fetching pools data"),
                                                StatusCode::INTERNAL_SERVER_ERROR,
                                            ));
                                        }
                                    };

                                    let mut pools = Vec::new();
                                    while let Some(pool_result) = pools_cursor.next().await {
                                        match pool_result {
                                            Ok(pool) => {
                                                let mut pool_doc = to_document(&pool).unwrap();
                                                pool_doc.remove("_id");
                                                pool_doc.remove("earnings_id");
                                                pools.push(pool_doc);
                                            }
                                            Err(e) => {
                                                eprintln!("Error processing pool data: {:?}", e);
                                                return Ok::<_, warp::Rejection>(with_status(
                                                    json(&"Error processing pool data"),
                                                    StatusCode::INTERNAL_SERVER_ERROR,
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
                                    return Ok::<_, warp::Rejection>(with_status(
                                        json(&"Error processing earnings data"),
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                    ));
                                }
                            }
                        }

                        if earnings_with_pools.is_empty() {
                            Ok::<_, warp::Rejection>(with_status(
                                json(&"No data found"),
                                StatusCode::NOT_FOUND,
                            ))
                        } else {
                            Ok::<_, warp::Rejection>(with_status(
                                json(&earnings_with_pools),
                                StatusCode::OK,
                            ))
                        }
                    } else {
                        let mut pools_cursor = match pools_collection.find(filter_doc) 
                            .sort(sort_doc.clone())
                            .skip(skip as u64)
                            .limit(limit)
                            .await {
                            Ok(cursor) => cursor,
                            Err(e) => {
                                eprintln!("Error fetching pools data: {:?}", e);
                                return Ok::<_, warp::Rejection>(with_status(
                                    json(&"Error fetching pools data"),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        };

                        let mut pools_data = Vec::new();

                        while let Some(pool_result) = pools_cursor.next().await {
                            match pool_result {
                                Ok(pool) => {
                                    let earnings_id = pool.earnings_id.clone(); 
                                    let earnings = match earnings_collection.find_one(doc! { "_id": earnings_id }).await {
                                        Ok(Some(earnings)) => earnings,
                                        Ok(None) => {
                                            eprintln!("No earnings found for pool: {:?}", pool);
                                            return Ok::<_, warp::Rejection>(with_status(
                                                json(&"No earnings found for pool"),
                                                StatusCode::NOT_FOUND,
                                            ));
                                        }
                                        Err(e) => {
                                            eprintln!("Error fetching earnings for pool: {:?}", e);
                                            return Ok::<_, warp::Rejection>(with_status(
                                                json(&"Error fetching earnings for pool"),
                                                StatusCode::INTERNAL_SERVER_ERROR,
                                            ));
                                        }
                                    };

                                    let mut pool_doc = to_document(&pool).unwrap();
                                    pool_doc.remove("_id");
                                    pool_doc.remove("earnings_id");
                                    
                                    pool_doc.insert("startTime", earnings.start_time);
                                    pool_doc.insert("endTime", earnings.end_time);
                                    
                                    pools_data.push(pool_doc);
                                }
                                Err(e) => {
                                    eprintln!("Error processing pool data: {:?}", e);
                                    return Ok::<_, warp::Rejection>(with_status(
                                        json(&"Error processing pool data"),
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                    ));
                                }
                            }
                        }

                        if pools_data.is_empty() {
                            Ok::<_, warp::Rejection>(with_status(
                                json(&"No data found"),
                                StatusCode::NOT_FOUND,
                            ))
                        } else {
                            Ok::<_, warp::Rejection>(with_status(
                                json(&pools_data),
                                StatusCode::OK,
                            ))
                        }
                    }
                }
            }
        })
}
