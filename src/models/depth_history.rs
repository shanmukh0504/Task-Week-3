use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistory {
    #[serde(skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<ObjectId>,
    pub pool: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "assetDepth")]
    pub asset_depth: i64,
    #[serde(rename = "assetPrice")]
    pub asset_price: f64,
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: f64,
    #[serde(rename = "liquidityUnits")]
    pub liquidity_units: i64,
    #[serde(rename = "membersCount")]
    pub members_count: i64,
    #[serde(rename = "runeDepth")]
    pub rune_depth: i64,
    #[serde(rename = "synthSupply")]
    pub synth_supply: i64,
    #[serde(rename = "synthUnits")]
    pub synth_units: i64,
    #[serde(rename = "units")]
    pub units: i64,
    #[serde(rename = "luvi")]
    pub luvi: f64,
}