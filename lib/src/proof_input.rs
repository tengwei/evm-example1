use alloy_primitives::Address;
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockWitnessProofInput {
    pub block_height: u64,
    pub user_data_delta_circuit_list: Vec<UserDataDeltaProofInput>,
    pub commitment: Vec<u8>,
    pub state_root_before: Vec<u8>,
    pub state_root_after: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDataDeltaProofInput {
    pub account_id: i64,
    pub address_before: Address,
    pub address_after: Address,
    pub state_root_before: Vec<u8>,
    pub state_root_after: Vec<u8>,
    pub balances_before: Vec<BalanceABI>,
    pub balances_after: Vec<BalanceABI>,
    pub positions_before: Vec<PositionABI>,
    pub positions_after: Vec<PositionABI>,
    pub merkle_proofs_before: Vec<Vec<u8>>,
    pub merkle_proofs_after: Vec<Vec<u8>>,
}

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct BalanceABI {
        bytes32 assetName;
        int256 balance;
        int256 maxWithdrawAmount;
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct PositionItemABI {
        int256 positionAmount;
        int256 entryPrice;
        int64 leverage;
        int256 unrealizedPnl;
        int256 returnOnEquity;
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct PositionABI {
        bytes32 symbolName;
        PositionItemABI[] positionItems;
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct UserDataABI {
        address userAddress;
        BalanceABI[] balances;
        PositionABI[] positions;
    }
}



