use alloy_primitives::{Address, I256};
use alloy_sol_types::sol;
use serde::{Deserialize, Deserializer, Serialize};
use crate::ACCOUNT_MERKLE_LEVELS;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockWitnessProofInput {
    pub block_height: u64,
    pub user_data_delta_circuit_list: Vec<UserDataDeltaProofInput>,
    // #[serde(deserialize_with = "deserialize_fixed_array")]
    pub commitment: [u8; 32],
    // #[serde(deserialize_with = "deserialize_fixed_array")]
    pub state_root_before: [u8; 32],
    // #[serde(deserialize_with = "deserialize_fixed_array")]
    pub state_root_after: [u8; 32],
    pub deposit_success_height: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDataDeltaProofInput {
    pub account_id: i64,
    pub address_before: Address,
    pub address_after: Address,
    // #[serde(deserialize_with = "deserialize_fixed_array")]
    pub state_root_before: [u8; 32],
    // #[serde(deserialize_with = "deserialize_fixed_array")]
    pub state_root_after: [u8; 32],

    pub abi_encode_before:[u8; 32],
    pub abi_encode_after:[u8; 32],

    // pub balances_before: Vec<BalanceABI>,
    // pub balances_after: Vec<BalanceABI>,
    // pub positions_before: Vec<PositionABI>,
    // pub positions_after: Vec<PositionABI>,
    pub merkle_proofs_before: [[u8; 32]; ACCOUNT_MERKLE_LEVELS],
    pub merkle_proofs_after: [[u8; 32]; ACCOUNT_MERKLE_LEVELS],
}

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct BalanceABI {
        bytes32 assetName;
        #[serde(deserialize_with = "deserialize_i256")]
        int256 balance;
        #[serde(deserialize_with = "deserialize_i256")]
        int256 maxWithdrawAmount;
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct PositionItemABI {
        #[serde(deserialize_with = "deserialize_i256")]
        int256 positionAmount;
        #[serde(deserialize_with = "deserialize_i256")]
        int256 entryPrice;
        uint64 leverage;
        #[serde(deserialize_with = "deserialize_i256")]
        int256 unrealizedPnl;
        #[serde(deserialize_with = "deserialize_i256")]
        int256 returnOnEquity;
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct PositionABI {
        bytes32 symbolName;
        PositionItemABI[] positionItems;
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct UserDataABI {
        address address;
        BalanceABI[] balances;
        PositionABI[] positions;
    }
}



fn deserialize_i256<'de, D>(deserializer: D) -> Result<I256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<I256>().map_err(serde::de::Error::custom)
}

fn deserialize_fixed_array<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<u8> = Deserialize::deserialize(deserializer)?;
    vec.try_into().map_err(|_| serde::de::Error::custom("could not convert slice to array"))
}




