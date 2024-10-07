use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolHistory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub earnings_id: ObjectId,
    #[serde(rename = "assetLiquidityFees")]
    pub asset_liquidity_fees: i64,
    pub earnings: i64,
    pub pool: String,
    pub rewards: i64,
    #[serde(rename = "runeLiquidityFees")]
    pub rune_liquidity_fees: i64,
    #[serde(rename = "saverEarning")]
    pub saver_earning: i64,
    #[serde(rename = "totalLiquidityFeesRune")]
    pub total_liquidity_fees_rune: i64,
}
