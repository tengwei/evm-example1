use alloy_sol_types::{sol, SolValue};
use tiny_keccak::{Hasher, Keccak};
// use derive_more::Display;
// use ethers::abi::encode;
// use ethers::abi::Token;
// use ethers::types::{Address, I256};
// use ethers::utils::keccak256;
// use serde::{Deserialize, Serialize};
//
// #[derive(Clone, Debug, Serialize, Deserialize,Display)]
// #[display("{self:?}")]
// pub struct UserDataABI {
//     pub address: Address,
//     pub balances: Vec<BalanceABI>,
//     pub positions: Vec<PositionABI>,
// }
//
// #[derive(Clone, Debug, Serialize, Deserialize,Display)]
// #[display("{self:?}")]
// pub struct BalanceABI {
//     pub asset_name: [u8; 32],
//     pub balance: I256,
//     pub max_withdraw_amount: I256,
// }
//
// #[derive(Clone, Debug, Serialize, Deserialize,Display)]
// #[display("{self:?}")]
// pub struct PositionItemABI {
//     pub position_amount: I256,
//     pub entry_price: I256,
//     pub leverage: i64,
//     pub unrealized_pnl: I256,
//     pub return_on_equity: I256,
// }
//
// #[derive(Clone, Debug, Serialize, Deserialize,Display)]
// #[display("{self:?}")]
// pub struct PositionABI {
//     pub symbol_name: [u8; 32],
//     pub positions: Vec<PositionItemABI>,
// }
//
// impl UserDataABI {
//     pub fn to_token(&self) -> Token {
//         Token::Tuple(vec![
//             Token::Address(self.address),
//             Token::Array(
//                 self.balances
//                     .iter()
//                     .map(|balance| {
//                         Token::Tuple(vec![
//                             Token::FixedBytes(balance.asset_name.to_vec()),
//                             Token::Int(balance.balance.into_raw()),
//                             Token::Int(balance.max_withdraw_amount.into_raw()),
//                         ])
//                     })
//                     .collect(),
//             ),
//             Token::Array(
//                 self.positions
//                     .iter()
//                     .map(|position| {
//                         Token::Tuple(vec![
//                             Token::FixedBytes(position.symbol_name.to_vec()),
//                             Token::Array(
//                                 position.positions
//                                     .iter()
//                                     .map(|item| {
//                                         Token::Tuple(vec![
//                                             Token::Int(item.position_amount.into_raw()),
//                                             Token::Int(item.entry_price.into_raw()),
//                                             Token::Int(item.leverage.into()),
//                                             Token::Int(item.unrealized_pnl.into_raw()),
//                                             Token::Int(item.return_on_equity.into_raw()),
//                                         ])
//                                     })
//                                     .collect(),
//                             ),
//                         ])
//                     })
//                     .collect(),
//             ),
//         ])
//     }
//
//     pub fn keccak256(&self) -> [u8; 32] {
//         let encoded = encode(&[self.to_token()]);
//         keccak256(&encoded)
//     }
// }

sol! {
    struct CommitmentABI {
        uint64 blockHeight;
        bytes32 stateRootBefore;
        bytes32 stateRootAfter;
    }
}
