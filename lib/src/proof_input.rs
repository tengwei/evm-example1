use alloy_primitives::Address;
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};
use crate::ACCOUNT_MERKLE_LEVELS;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockWitnessProofInput {
    pub block_height: u64,
    pub user_data_delta_circuit_list: Vec<UserDataDeltaProofInput>,
    pub commitment: [u8; 32],
    pub state_root_before: [u8; 32],
    pub state_root_after: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDataDeltaProofInput {
    pub account_id: i64,
    pub address_before: Address,
    pub address_after: Address,
    pub state_root_before: [u8; 32],
    pub state_root_after: [u8; 32],
    pub balances_before: Vec<BalanceABI>,
    pub balances_after: Vec<BalanceABI>,
    pub positions_before: Vec<PositionABI>,
    pub positions_after: Vec<PositionABI>,
    pub merkle_proofs_before: [[u8; 32]; ACCOUNT_MERKLE_LEVELS],
    pub merkle_proofs_after: [[u8; 32]; ACCOUNT_MERKLE_LEVELS],
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

sol! {
    struct CommitmentABI {
        uint64 blockHeight;
        bytes32 stateRootBefore;
        bytes32 stateRootAfter;
    }
}




