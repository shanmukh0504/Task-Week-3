use crate::models::runepool_history::RunePoolHistory;
use bson::{doc, to_document};
use futures::stream::StreamExt;
use mongodb::Collection;
use serde::Deserialize;
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use warp::Filter;

#[derive(Debug, Deserialize)]
pub struct RunePoolQueryParams {
    start_time: Option<i64>,
    end_time: Option<i64>,
    page: Option<u32>,
    limit: Option<u32>,
    sort_by: Option<String>,
    order: Option<String>,
}

pub fn runepool_history_route(
    collection: Collection<RunePoolHistory>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("runepool-history")
        .and(warp::get())
        .and(warp::query::<RunePoolQueryParams>())
        .and_then({
            let collection = collection.clone();
            move |params: RunePoolQueryParams| {
                let collection = collection.clone();
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

                    let filter = if !filter_conditions.is_empty() {
                        doc! { "$and": filter_conditions }
                    } else {
                        doc! {}
                    };

                    let limit = params.limit.unwrap_or(10).clamp(1, 100) as i64;
                    let skip =
                        ((params.page.unwrap_or(1).max(1) - 1) * params.limit.unwrap_or(10)) as u64;

                    let sort_doc = if let Some(sort_by) = params.sort_by {
                        let sort_order = match params.order.as_deref() {
                            Some("asc") => 1,
                            Some("desc") | _ => -1,
                        };
                        doc! { &sort_by: sort_order }
                    } else {
                        doc! { "startTime": -1 }
                    };

                    let mut cursor = match collection
                        .find(filter)
                        .sort(sort_doc)
                        .skip(skip)
                        .limit(limit)
                        .await
                    {
                        Ok(cursor) => cursor,
                        Err(e) => {
                            eprintln!("Error fetching data: {:?}", e);
                            return Ok::<_, warp::Rejection>(with_status(
                                json(&"Error fetching data"),
                                StatusCode::INTERNAL_SERVER_ERROR,
                            ));
                        }
                    };

                    let mut histories = Vec::new();
                    while let Some(result) = cursor.next().await {
                        match result {
                            Ok(raw_history) => {
                                let mut response = to_document(&raw_history).unwrap();
                                response.remove("_id");
                                histories.push(response);
                            }
                            Err(e) => {
                                eprintln!("Error processing data: {:?}", e);
                                return Ok::<_, warp::Rejection>(with_status(
                                    json(&"Error processing data"),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        }
                    }

                    if histories.is_empty() {
                        Ok::<_, warp::Rejection>(with_status(
                            json(&"No data found"),
                            StatusCode::NOT_FOUND,
                        ))
                    } else {
                        Ok::<_, warp::Rejection>(with_status(json(&histories), StatusCode::OK))
                    }
                }
            }
        })
}