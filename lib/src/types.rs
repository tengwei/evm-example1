use base64::{engine::general_purpose, Engine as _};
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockWitnessCircuit {
    #[serde(rename = "blockHeight")]
    pub block_height: u64,
    #[serde(rename = "userDataDeltaCircuitList")]
    pub user_data_delta_circuit_list: Vec<UserDataDeltaCircuit>,
    #[serde(rename = "commitment", with = "serde_bytes")]
    pub commitment: Vec<u8>,
    #[serde(rename = "stateRootBefore", with = "serde_bytes")]
    pub state_root_before: Vec<u8>,
    #[serde(rename = "stateRootAfter", with = "serde_bytes")]
    pub state_root_after: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDataDeltaCircuit {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "addressBefore")]
    pub address_before: String,
    #[serde(rename = "addressAfter")]
    pub address_after: String,
    #[serde(rename = "stateRootBefore", with = "serde_bytes")]
    pub state_root_before: Vec<u8>,
    #[serde(rename = "stateRootAfter", with = "serde_bytes")]
    pub state_root_after: Vec<u8>,
    #[serde(rename = "balancesBefore")]
    pub balances_before: Vec<Balance>,
    #[serde(rename = "balancesAfter")]
    pub balances_after: Vec<Balance>,
    #[serde(rename = "positionsBefore")]
    pub positions_before: Vec<Position>,
    #[serde(rename = "positionsAfter")]
    pub positions_after: Vec<Position>,
    #[serde(
        rename = "merkleProofsBefore",
        serialize_with = "serialize_nested_bytes",
        deserialize_with = "deserialize_nested_bytes"
    )]
    pub merkle_proofs_before: Vec<Vec<u8>>,
    #[serde(
        rename = "merkleProofsAfter",
        serialize_with = "serialize_nested_bytes",
        deserialize_with = "deserialize_nested_bytes"
    )]
    pub merkle_proofs_after: Vec<Vec<u8>>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Balance {
    #[serde(rename = "assetName")]
    pub asset_name: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "withdrawAmount")]
    pub withdraw_amount: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "symbolName")]
    pub symbol_name: String,
    #[serde(rename = "positionItems")]
    pub position_items: Vec<PositionItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionItem {
    #[serde(rename = "positionAmount")]
    pub position_amount: String,
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    #[serde(rename = "leverage")]
    pub leverage: i64,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "returnOnEquity")]
    pub return_on_equity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    #[serde(rename = "stateRootBefore", with = "serde_bytes")]
    pub state_root_before: Vec<u8>, // 自动解码 Base64 字符串
    #[serde(rename = "stateRootAfter", with = "serde_bytes")]
    pub state_root_after: Vec<u8>,  // 自动解码 Base64 字符串
}

fn serialize_nested_bytes<S>(data: &Vec<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let base64_strings: Vec<String> = data
        .iter()
        .map(|d| general_purpose::STANDARD.encode(d))
        .collect();
    base64_strings.serialize(serializer)
}

fn deserialize_nested_bytes<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: Vec<String> = Deserialize::deserialize(deserializer)?;
    encoded
        .into_iter()
        .map(|s| general_purpose::STANDARD.decode(&s).map_err(serde::de::Error::custom))
        .collect()
}