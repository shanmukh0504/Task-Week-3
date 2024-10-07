use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EarningsHistory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "avgNodeCount")]
    pub avg_node_count: f64,
    #[serde(rename = "blockRewards")]
    pub block_rewards: i64,
    #[serde(rename = "bondingEarnings")]
    pub bonding_earnings: i64,
    pub earnings: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "liquidityEarnings")]
    pub liquidity_earnings: i64,
    #[serde(rename = "liquidityFees")]
    pub liquidity_fees: i64,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
}
