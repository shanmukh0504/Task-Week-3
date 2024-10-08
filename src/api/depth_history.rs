use crate::models::depth_history::{DepthHistory, Metadata};
use actix_web::{web, HttpResponse, Responder};
use bson::{doc, to_document};
use futures::stream::StreamExt;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistoryQueryParams {
    from: Option<i64>,
    to: Option<i64>,
    pool: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
    sort_by: Option<String>,
    order: Option<String>,
}

#[derive(Debug, Serialize)]
struct ResponseWithMeta {
    data: Vec<bson::Document>,
    meta: Metadata,
}

pub async fn depth_history_route(
    query: web::Query<DepthHistoryQueryParams>,
    collection: web::Data<Collection<DepthHistory>>,
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

    if let Some(pool) = params.pool {
        filter_conditions.push(doc! { "pool": pool });
    }

    let filter = if !filter_conditions.is_empty() {
        doc! { "$and": filter_conditions }
    } else {
        doc! {}
    };

    let limit = params.limit.unwrap_or(10).clamp(1, 100) as i64;
    let skip = ((params.page.unwrap_or(1).max(1) - 1) * params.limit.unwrap_or(10)) as u64;

    let sort_doc = if let Some(sort_by) = params.sort_by.clone() {
        let sort_order = match params.order.as_deref() {
            Some("asc") => 1,
            Some("desc") | _ => -1,
        };
        doc! { sort_by: sort_order }
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
    let mut total_asset_depth = 0;
    let mut total_lp_units = 0;
    let mut total_member_count = 0;
    let mut total_rune_depth = 0;
    let mut total_synth_units = 0;

    let mut first_start_time = None;
    let mut last_end_time = None;

    let mut start_asset_depth = 0;
    let mut start_lp_units = 0;
    let mut start_member_count = 0;
    let mut start_rune_depth = 0;
    let mut start_synth_units = 0;

    let mut end_asset_depth = 0;
    let mut end_lp_units = 0;
    let mut end_member_count = 0;
    let mut end_rune_depth = 0;
    let mut end_synth_units = 0;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(raw_history) => {
                let mut response = to_document(&raw_history).unwrap();
                response.remove("_id");

                let start_time = response.get_i64("startTime").unwrap_or(0);
                let end_time = response.get_i64("endTime").unwrap_or(0);
                let asset_depth = response.get_i64("assetDepth").unwrap_or(0);
                let lp_units = response.get_i64("liquidityUnits").unwrap_or(0);
                let member_count = response.get_i64("membersCount").unwrap_or(0);
                let rune_depth = response.get_i64("runeDepth").unwrap_or(0);
                let synth_units = response.get_i64("synthUnits").unwrap_or(0);

                if first_start_time.is_none() {
                    first_start_time = Some(start_time);
                    start_asset_depth = asset_depth;
                    start_lp_units = lp_units;
                    start_member_count = member_count;
                    start_rune_depth = rune_depth;
                    start_synth_units = synth_units;
                }

                last_end_time = Some(end_time);
                end_asset_depth = asset_depth;
                end_lp_units = lp_units;
                end_member_count = member_count;
                end_rune_depth = rune_depth;
                end_synth_units = synth_units;

                total_asset_depth += asset_depth;
                total_lp_units += lp_units;
                total_member_count += member_count;
                total_rune_depth += rune_depth;
                total_synth_units += synth_units;

                histories.push(response);
            }
            Err(e) => {
                eprintln!("Error processing data: {:?}", e);
                return HttpResponse::InternalServerError().json("Error processing data");
            }
        }
    }

    if histories.is_empty() {
        return HttpResponse::NotFound().json("No data found");
    }

    let count = histories.len() as i64;
    let avg_asset_depth = total_asset_depth / count;
    let avg_lp_units = total_lp_units / count;
    let avg_member_count = total_member_count / count;
    let avg_rune_depth = total_rune_depth / count;
    let avg_synth_units = total_synth_units / count;

    let metadata = Metadata {
        start_time: first_start_time.unwrap_or(0).to_string(),
        end_time: last_end_time.unwrap_or(0).to_string(),
        start_asset_depth: start_asset_depth.to_string(),
        end_asset_depth: end_asset_depth.to_string(),
        avg_asset_depth: avg_asset_depth.to_string(),
        start_lp_units: start_lp_units.to_string(),
        end_lp_units: end_lp_units.to_string(),
        avg_lp_units: avg_lp_units.to_string(),
        start_member_count: start_member_count.to_string(),
        end_member_count: end_member_count.to_string(),
        avg_member_count: avg_member_count.to_string(),
        start_rune_depth: start_rune_depth.to_string(),
        end_rune_depth: end_rune_depth.to_string(),
        avg_rune_depth: avg_rune_depth.to_string(),
        start_synth_units: start_synth_units.to_string(),
        end_synth_units: end_synth_units.to_string(),
        avg_synth_units: avg_synth_units.to_string(),
    };

    let response = ResponseWithMeta {
        data: histories,
        meta: metadata,
    };

    HttpResponse::Ok().json(response)
}
