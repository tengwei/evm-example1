use base64::{engine::general_purpose, Engine as _};
use derive_more::Display;
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[display("{self:?}")]
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

#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[display("{self:?}")]
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


#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct Balance {
    #[serde(rename = "assetName")]
    pub asset_name: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "withdrawAmount")]
    pub withdraw_amount: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct Position {
    #[serde(rename = "symbolName")]
    pub symbol_name: String,
    #[serde(rename = "positionItems")]
    pub position_items: Vec<PositionItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[display("{self:?}")]
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

// fn serialize_nested_bytes<S>(nested: &Vec<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let serialized: Vec<serde_bytes::Bytes> = nested.iter().map(|v| serde_bytes::Bytes::new(v)).collect();
//     serialized.serialize(serializer)
//
//
// }

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


// fn serialize_balances<S>(balances: &Vec<Balance>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let serialized: Vec<String> = balances.iter().map(|balance| serde_json::to_string(balance).unwrap()).collect();
//     serialized.serialize(serializer)
// }
//
// fn deserialize_balances<'de, D>(deserializer: D) -> Result<Vec<Balance>, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let deserialized: Vec<String> = Deserialize::deserialize(deserializer)?;
//     deserialized
//         .into_iter()
//         .map(|s| serde_json::from_str(&s).map_err(de::Error::custom))
//         .collect()
// }

// fn serialize_positions<S>(positions: &Vec<Position>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let serialized: Vec<String> = positions
//         .iter()
//         .map(|position| serde_json::to_string(position).unwrap())
//         .collect();
//     serialized.serialize(serializer)
// }
//
// fn deserialize_positions<'de, D>(deserializer: D) -> Result<Vec<Position>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let deserialized: Vec<String> = Deserialize::deserialize(deserializer)?;
//     deserialized
//         .into_iter()
//         .map(|s| serde_json::from_str(&s).map_err(de::Error::custom))
//         .collect()
// }


// fn deserialize_nested_bytes<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let deserialized: Vec<serde_bytes::ByteBuf> = Deserialize::deserialize(deserializer)?;
//     Ok(deserialized.into_iter().map(|buf| buf.into_vec()).collect())
// }