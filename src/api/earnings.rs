use actix_web::{web, HttpResponse, Responder};
use mongodb::Collection;
use crate::models::{earnings_history::EarningsHistory, pools_history::PoolHistory};
use mongodb::bson::{doc, to_document};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsWithPoolsQueryParams {
    from: Option<i64>,
    to: Option<i64>,
    summary: Option<bool>,
    page: Option<u32>,
    limit: Option<u32>,
    sort_by: Option<String>,
    order: Option<String>,
}

pub async fn earnings_with_pools_route(
    query: web::Query<EarningsWithPoolsQueryParams>,
    earnings_collection: web::Data<Collection<EarningsHistory>>,
    pools_collection: web::Data<Collection<PoolHistory>>,
) -> impl Responder {
    let params = query.into_inner();
    let mut filter_conditions = Vec::new();

    if let (Some(start), Some(end)) = (params.from, params.to) {
        if start >= end {
            return HttpResponse::BadRequest().json("start_time must be less than end_time");
        }
        filter_conditions.push(doc! { "startTime": { "$gte": start } });
        filter_conditions.push(doc! { "endTime": { "$lte": end } });
    } else if let Some(start) = params.from {
        filter_conditions.push(doc! { "startTime": { "$gte": start } });
    } else if let Some(end) = params.to {
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
                return HttpResponse::InternalServerError().json("Error fetching earnings data");
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
                        .await {
                        Ok(cursor) => cursor,
                        Err(e) => {
                            eprintln!("Error fetching pools data: {:?}", e);
                            return HttpResponse::InternalServerError().json("Error fetching pools data");
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
                                return HttpResponse::InternalServerError().json("Error processing pool data");
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
                    return HttpResponse::InternalServerError().json("Error processing earnings data");
                }
            }
        }

        if earnings_with_pools.is_empty() {
            HttpResponse::NotFound().json("No data found")
        } else {
            HttpResponse::Ok().json(earnings_with_pools)
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
                return HttpResponse::InternalServerError().json("Error fetching pools data");
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
                            return HttpResponse::NotFound().json("No earnings found for pool");
                        }
                        Err(e) => {
                            eprintln!("Error fetching earnings for pool: {:?}", e);
                            return HttpResponse::InternalServerError().json("Error fetching earnings for pool");
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
                    return HttpResponse::InternalServerError().json("Error processing pool data");
                }
            }
        }

        if pools_data.is_empty() {
            HttpResponse::NotFound().json("No data found")
        } else {
            HttpResponse::Ok().json(pools_data)
        }
    }
}