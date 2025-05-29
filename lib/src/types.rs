
use derive_more::Display;
use serde::{Deserialize, Serialize};
use num_bigint::BigInt;
use serde::de::{self, Deserializer};


#[derive(Clone, Debug, Serialize, Deserialize,Display)]
#[display("{self:?}")]
pub struct BlockWitnessCircuit {
    #[serde(rename = "blockHeight")]
    pub block_height: u64,
    #[serde(rename = "userDataDeltaCircuitList")]
    pub user_data_delta_circuit_list: Vec<UserDataDeltaCircuit>,
    #[serde(rename = "commitment",with = "serde_bytes")]
    pub commitment: Vec<u8>,
    #[serde(rename = "stateRootBefore",with = "serde_bytes")]
    pub state_root_before: Vec<u8>,
    #[serde(rename = "stateRootAfter",with = "serde_bytes")]
    pub state_root_after: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize,Display)]
#[display("{self:?}")]
pub struct UserDataDeltaCircuit {
    #[serde(rename = "accountId")]
    pub account_id: i32,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "stateRootBefore",with = "serde_bytes")]
    pub state_root_before: Vec<u8>,
    #[serde(rename = "stateRootAfter",with = "serde_bytes")]
    pub state_root_after: Vec<u8>,
    #[serde(rename = "balances",serialize_with = "serialize_balances", deserialize_with = "deserialize_balances")]
    pub balances: Vec<Balance>,
    #[serde(rename = "positions",serialize_with = "serialize_positions", deserialize_with = "deserialize_positions")]
    pub positions: Vec<Position>,
    #[serde(rename = "merkleProofsBefore",serialize_with = "serialize_nested_bytes", deserialize_with = "deserialize_nested_bytes")]
    pub merkle_proofs_before: Vec<Vec<u8>>, // Assuming AccountMerkleLevels is dynamic
    #[serde(rename = "merkleProofsAfter",serialize_with = "serialize_nested_bytes", deserialize_with = "deserialize_nested_bytes")]
    pub merkle_proofs_after: Vec<Vec<u8>>, // Assuming AccountMerkleLevels is dynamic
}


#[derive(Clone, Debug, Serialize, Deserialize,Display)]
#[display("{self:?}")]
pub struct Balance {
    #[serde(rename = "assetName")]
    pub asset_name: String,
    #[serde(rename = "beforeBalance")]
    pub before_balance: Option<BigInt>,
    #[serde(rename = "beforeWithdrawAmount")]
    pub before_withdraw_amount: Option<BigInt>,
    #[serde(rename = "afterBalance")]
    pub after_balance: Option<BigInt>,
    #[serde(rename = "afterWithdrawAmount")]
    pub after_withdraw_amount: Option<BigInt>,
}

#[derive(Clone, Debug, Serialize, Deserialize,Display)]
#[display("{self:?}")]
pub struct Position {
    #[serde(rename = "symbolName")]
    pub symbol_name: String,
    #[serde(rename = "beforeBothPosition")]
    pub before_both_position: Option<PositionItem>,
    #[serde(rename = "beforeLongPosition")]
    pub before_long_position: Option<PositionItem>,
    #[serde(rename = "beforeShortPosition")]
    pub before_short_position: Option<PositionItem>,
    #[serde(rename = "afterBothPosition")]
    pub after_both_position: Option<PositionItem>,
    #[serde(rename = "afterLongPosition")]
    pub after_long_position: Option<PositionItem>,
    #[serde(rename = "afterShortPosition")]
    pub after_short_position: Option<PositionItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize,Display)]
#[display("{self:?}")]
pub struct PositionItem {
    #[serde(rename = "positionAmount")]
    pub position_amount: Option<BigInt>,
    #[serde(rename = "entryPrice")]
    pub entry_price: Option<BigInt>,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(rename = "isolated")]
    pub isolated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    #[serde(rename = "stateRootBefore",with = "serde_bytes")]
    pub state_root_before: Vec<u8>, // 自动解码 Base64 字符串
    #[serde(rename = "stateRootAfter",with = "serde_bytes")]
    pub state_root_after: Vec<u8>,  // 自动解码 Base64 字符串
}

fn serialize_nested_bytes<S>(nested: &Vec<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let serialized: Vec<serde_bytes::Bytes> = nested.iter().map(|v| serde_bytes::Bytes::new(v)).collect();
    serialized.serialize(serializer)
}

fn deserialize_nested_bytes<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deserialized: Vec<serde_bytes::ByteBuf> = Deserialize::deserialize(deserializer)?;
    Ok(deserialized.into_iter().map(|buf| buf.into_vec()).collect())
}

fn serialize_balances<S>(balances: &Vec<Balance>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let serialized: Vec<String> = balances.iter().map(|balance| serde_json::to_string(balance).unwrap()).collect();
    serialized.serialize(serializer)
}

fn deserialize_balances<'de, D>(deserializer: D) -> Result<Vec<Balance>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deserialized: Vec<String> = Deserialize::deserialize(deserializer)?;
    deserialized
        .into_iter()
        .map(|s| serde_json::from_str(&s).map_err(de::Error::custom))
        .collect()
}

fn serialize_positions<S>(positions: &Vec<Position>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let serialized: Vec<String> = positions
        .iter()
        .map(|position| serde_json::to_string(position).unwrap())
        .collect();
    serialized.serialize(serializer)
}

fn deserialize_positions<'de, D>(deserializer: D) -> Result<Vec<Position>, D::Error>
where
    D: Deserializer<'de>,
{
    let deserialized: Vec<String> = Deserialize::deserialize(deserializer)?;
    deserialized
        .into_iter()
        .map(|s| serde_json::from_str(&s).map_err(de::Error::custom))
        .collect()
}