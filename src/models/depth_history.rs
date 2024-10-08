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

#[derive(Debug, Serialize)]
pub struct Metadata {
    #[serde(rename = "startTime")]
    pub start_time: String,
    
    #[serde(rename = "endTime")]
    pub end_time: String,
    
    #[serde(rename = "startAssetDepth")]
    pub start_asset_depth: String,
    
    #[serde(rename = "endAssetDepth")]
    pub end_asset_depth: String,
    
    #[serde(rename = "avgAssetDepth")]
    pub avg_asset_depth: String,
    
    #[serde(rename = "startLPUnits")]
    pub start_lp_units: String,
    
    #[serde(rename = "endLPUnits")]
    pub end_lp_units: String,
    
    #[serde(rename = "avgLPUnits")]
    pub avg_lp_units: String,
    
    #[serde(rename = "startMemberCount")]
    pub start_member_count: String,
    
    #[serde(rename = "endMemberCount")]
    pub end_member_count: String,
    
    #[serde(rename = "avgMemberCount")]
    pub avg_member_count: String,
    
    #[serde(rename = "startRuneDepth")]
    pub start_rune_depth: String,
    
    #[serde(rename = "endRuneDepth")]
    pub end_rune_depth: String,
    
    #[serde(rename = "avgRuneDepth")]
    pub avg_rune_depth: String,
    
    #[serde(rename = "startSynthUnits")]
    pub start_synth_units: String,
    
    #[serde(rename = "endSynthUnits")]
    pub end_synth_units: String,
    
    #[serde(rename = "avgSynthUnits")]
    pub avg_synth_units: String,
}
