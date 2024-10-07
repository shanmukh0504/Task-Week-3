use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapHistory {
    #[serde(skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<ObjectId>,
    pub pool: String,
    #[serde(rename = "averageSlip")]
    pub average_slip: f64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "synthMintAverageSlip")]
    pub synth_mint_average_slip: f64,
    #[serde(rename = "synthMintCount")]
    pub synth_mint_count: i64,
    #[serde(rename = "synthMintFees")]
    pub synth_mint_fees: i64,
    #[serde(rename = "synthMintVolume")]
    pub synth_mint_volume: i64,
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: f64,
    #[serde(rename = "synthRedeemAverageSlip")]
    pub synth_redeem_average_slip: f64,
    #[serde(rename = "synthRedeemCount")]
    pub synth_redeem_count: i64,
    #[serde(rename = "synthRedeemFees")]
    pub synth_redeem_fees: i64,
    #[serde(rename = "synthRedeemVolume")]
    pub synth_redeem_volume: i64,
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: f64,
    #[serde(rename = "toAssetAverageSlip")]
    pub to_asset_average_slip: f64,
    #[serde(rename = "toAssetCount")]
    pub to_asset_count: i64,
    #[serde(rename = "toAssetFees")]
    pub to_asset_fees: i64,
    #[serde(rename = "toAssetVolume")]
    pub to_asset_volume: i64,
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: f64,
    #[serde(rename = "toRuneAverageSlip")]
    pub to_rune_average_slip: f64,
    #[serde(rename = "toRuneCount")]
    pub to_rune_count: i64,
    #[serde(rename = "toRuneFees")]
    pub to_rune_fees: i64,
    #[serde(rename = "toRuneVolume")]
    pub to_rune_volume: i64,
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: f64,
    #[serde(rename = "totalCount")]
    pub total_count: i64,
    #[serde(rename = "totalFees")]
    pub total_fees: i64,
    #[serde(rename = "totalVolume")]
    pub total_volume: i64,
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: f64,
}
