use crate::models::swaps_history::SwapHistory;
use bson::{doc, to_document};
use futures::stream::StreamExt;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Responder};

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapQueryParams {
    from: Option<i64>,
    to: Option<i64>,
    page: Option<u32>,
    limit: Option<u32>,
    sort_by: Option<String>,
    order: Option<String>,
}

pub async fn swaps_history_route(
    query: web::Query<SwapQueryParams>,
    collection: web::Data<Collection<SwapHistory>>,
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

    let filter = if !filter_conditions.is_empty() {
        doc! { "$and": filter_conditions }
    } else {
        doc! {}
    };

    let limit = params.limit.unwrap_or(10).clamp(1, 100) as i64;
    let skip = ((params.page.unwrap_or(1).max(1) - 1) * params.limit.unwrap_or(10)) as u64;

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
            return HttpResponse::InternalServerError().json("Error fetching data");
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
                return HttpResponse::InternalServerError().json("Error processing data");
            }
        }
    }

    if histories.is_empty() {
        HttpResponse::NotFound().json("No data found")
    } else {
        HttpResponse::Ok().json(histories)
    }
}
