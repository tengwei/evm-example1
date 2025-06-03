use std::str::FromStr;
use alloy_primitives::{Address, FixedBytes};
use alloy_sol_types::SolValue;
use base64::{engine::general_purpose, Engine as _};
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use crate::proof_input::{BalanceABI, BlockWitnessProofInput, PositionABI, PositionItemABI, UserDataABI, UserDataDeltaProofInput};
use crate::{vec_to_array, vec_to_bytes32};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockWitnessCircuit {
    #[serde(rename = "blockHeight")]
    pub block_height: u64,
    #[serde(rename = "userDataDeltaCircuitList")]
    pub user_data_delta_circuit_list: Vec<UserDataDeltaCircuit>,
    #[serde(rename = "commitment", deserialize_with = "custom_base64_decode")]
    pub commitment: Vec<u8>,
    #[serde(rename = "stateRootBefore", deserialize_with = "custom_base64_decode")]
    pub state_root_before: Vec<u8>,
    #[serde(rename = "stateRootAfter", deserialize_with = "custom_base64_decode")]
    pub state_root_after: Vec<u8>,
    #[serde(rename = "depositSuccessHeight")]
    pub deposit_success_height: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDataDeltaCircuit {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "addressBefore")]
    pub address_before: String,
    #[serde(rename = "addressAfter")]
    pub address_after: String,
    #[serde(rename = "stateRootBefore", deserialize_with = "custom_base64_decode")]
    pub state_root_before: Vec<u8>,
    #[serde(rename = "stateRootAfter", deserialize_with = "custom_base64_decode")]
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
    pub leverage: u64,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "returnOnEquity")]
    pub return_on_equity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    #[serde(rename = "stateRootBefore", deserialize_with = "custom_base64_decode")]
    pub state_root_before: Vec<u8>, // 自动解码 Base64 字符串
    #[serde(rename = "stateRootAfter", deserialize_with = "custom_base64_decode")]
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


impl From<Balance> for BalanceABI {
    fn from(balance: Balance) -> Self {
        BalanceABI {
            assetName: {
                let mut name = [0u8; 32];
                let bytes = balance.asset_name.as_bytes();
                name[..bytes.len()].copy_from_slice(bytes);
                FixedBytes::from(name)
            },
            balance: balance.balance.parse().unwrap_or_default(),
            maxWithdrawAmount: balance.withdraw_amount.parse().unwrap_or_default(),
        }
    }
}

impl From<Position> for PositionABI {
    fn from(position: Position) -> Self {
        PositionABI {
            symbolName: {
                let mut name = [0u8; 32];
                let bytes = position.symbol_name.as_bytes();
                name[..bytes.len()].copy_from_slice(bytes);
                FixedBytes::from(name)
            },
            positionItems: position
                .position_items
                .into_iter()
                .map(|item| PositionItemABI {
                    positionAmount: item.position_amount.parse().unwrap_or_default(),
                    entryPrice: item.entry_price.parse().unwrap_or_default(),
                    leverage: item.leverage,
                    unrealizedPnl: item.unrealized_pnl.parse().unwrap_or_default(),
                    returnOnEquity: item.return_on_equity.parse().unwrap_or_default(),
                })
                .collect(),
        }
    }
}

impl From<BlockWitnessCircuit> for BlockWitnessProofInput {
    fn from(circuit: BlockWitnessCircuit) -> Self {
        BlockWitnessProofInput {
            block_height: circuit.block_height,
            user_data_delta_circuit_list: circuit
                .user_data_delta_circuit_list
                .into_iter()
                .map(|delta| UserDataDeltaProofInput {
                    account_id: delta.account_id,
                    address_before: Address::from_str(&delta.address_before).unwrap_or_default(),
                    address_after: Address::from_str(&delta.address_after).unwrap_or_default(),
                    state_root_before: vec_to_bytes32(delta.state_root_before),
                    state_root_after: vec_to_bytes32(delta.state_root_after),
                    abi_encode_before:vec_to_bytes32(UserDataABI { address:Address::from_str(&delta.address_before).unwrap_or_default(), balances:delta
                        .balances_before
                        .into_iter()
                        .map(|balance| balance.into())
                        .collect(), positions:delta
                        .positions_before
                        .into_iter()
                        .map(|position| position.into())
                        .collect() }.abi_encode()),
                    abi_encode_after:vec_to_bytes32(UserDataABI { address:Address::from_str(&delta.address_after).unwrap_or_default(), balances:delta
                        .balances_after
                        .into_iter()
                        .map(|balance| balance.into())
                        .collect(), positions:delta
                        .positions_after
                        .into_iter()
                        .map(|position| position.into())
                        .collect() }.abi_encode()),
                    // balances_before: delta
                    //     .balances_before
                    //     .into_iter()
                    //     .map(|balance| balance.into())
                    //     .collect(),
                    // balances_after: delta
                    //     .balances_after
                    //     .into_iter()
                    //     .map(|balance| balance.into())
                    //     .collect(),
                    // positions_before: delta
                    //     .positions_before
                    //     .into_iter()
                    //     .map(|position| position.into())
                    //     .collect(),
                    // positions_after: delta
                    //     .positions_after
                    //     .into_iter()
                    //     .map(|position| position.into())
                    //     .collect(),
                    merkle_proofs_before: vec_to_array(delta.merkle_proofs_before),
                    merkle_proofs_after: vec_to_array(delta.merkle_proofs_after),
                })
                .collect(),
            commitment: vec_to_bytes32(circuit.commitment),
            state_root_before: vec_to_bytes32(circuit.state_root_before),
            state_root_after: vec_to_bytes32(circuit.state_root_after),
            deposit_success_height:circuit.deposit_success_height
        }
    }
}

pub fn custom_base64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: String = Deserialize::deserialize(deserializer)?;
    base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .map_err(serde::de::Error::custom)
}
