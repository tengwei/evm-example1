#![feature(portable_simd)]

pub mod types;
mod abi;
pub mod proof_input;

use std::fs;
use alloy_primitives::Address;
use alloy_sol_types::{sol, SolValue};
use tiny_keccak::{Hasher, Keccak};
use crate::abi::CommitmentABI;
use crate::proof_input::{BalanceABI, BlockWitnessProofInput, PositionABI, UserDataABI};

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

sol! {
    struct User {
        address addr;
        uint256 balance;
    }
}





pub fn verify(block_witness: BlockWitnessProofInput) -> () {
    let size = block_witness.user_data_delta_circuit_list.len();
    assert!(compare(block_witness.commitment, keccak256_for_commitment(block_witness.block_height, block_witness.state_root_before, block_witness.state_root_after)));

    if size == 0 {
        //state_root_before must be equal state_root_after
        assert_eq!(block_witness.state_root_before, block_witness.state_root_after, "Mismatch State Root!");
    } else {
        for user_data in block_witness.user_data_delta_circuit_list.iter() {
            println!("Account ID: {}", user_data.account_id);
            println!("Address Before: {:?}", user_data.address_before);
            println!("Address After: {:?}", user_data.address_after);
            println!("State Root Before: {:?}", user_data.state_root_before);
            println!("State Root After: {:?}", user_data.state_root_after);

            let hash_before = generate_leaf_hash(user_data.address_before, user_data.balances_before, user_data.positions_before);
            let is_valid_before = verify_sparse_merkle_root(hash_before, user_data.merkle_proofs_before, user_data.state_root_before);
            assert!(is_valid_before, "Mismatch State Root!");

            let hash_after = generate_leaf_hash(user_data.address_after, user_data.balances_after, user_data.positions_after);
            let is_valid_after = verify_sparse_merkle_root(hash_after, user_data.merkle_proofs_after, user_data.state_root_after);
            assert!(is_valid_after, "Mismatch State Root!");
        }
    }
}

fn keccak256_for_commitment(block_height: u64, state_root_before: Vec<u8>, state_root_after: Vec<u8>) -> [u8; 32] {
    // ABI encode
    let encoded = CommitmentABI {
        blockHeight: block_height,
        stateRootBefore: vec_to_bytes32(state_root_before).into(),
        stateRootAfter: vec_to_bytes32(state_root_after).into(),
    }.abi_encode();

    // Keccak256 hash
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(&encoded);
    hasher.finalize(&mut output);

    output
}

fn vec_to_bytes32(vec: Vec<u8>) -> [u8; 32] {
    let mut bytes32 = [0u8; 32];
    let len = vec.len().min(32); // Make sure it does not exceed 32 bytes
    bytes32[..len].copy_from_slice(&vec[..len]);
    bytes32
}

fn compare(vec: Vec<u8>, arr: [u8; 32]) -> bool {
    // Check vec length to avoid panic
    if vec.len() != 32 {
        return false;
    }
    // Compare contents (as_slice converts to &[u8], which automatically supports comparison with [u8; 32])
    arr == vec[..]
}

fn generate_leaf_hash(address: Address, balances: Vec<BalanceABI>, positions: Vec<PositionABI>) -> [u8; 32] {
    // ABI encode
    let encoded = UserDataABI { userAddress: address, balances, positions }.abi_encode();

    // Keccak256 hash
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(&encoded);
    hasher.finalize(&mut output);

    output
}

fn verify_sparse_merkle_root(
    account_id:u64,
    leaf_hash: [u8; 32],
    merkle_proofs: Vec<Vec<u8>>,
    state_root: Vec<u8>,
) -> bool {
    let mut current_hash = leaf_hash;
    //todo
    let path = leaf_id_to_path(account_id, 32);

    for (proof_hash, is_right) in merkle_proofs.iter().zip(path.iter()) {
        current_hash = if *is_right {
            hash_node(vec_to_bytes32(*proof_hash), current_hash)
        } else {
            hash_node(&current_hash, proof_hash)
        };
    }

    current_hash == vec_to_bytes32(state_root)
}

fn hash_node(left: &[u8; 32], right: &Vec<u8>) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(left);
    hasher.update(right);
    hasher.finalize(&mut output);
    output
}

fn leaf_id_to_path(leaf_id: u64, tree_height: u32) -> Vec<bool> {
    let mut path = Vec::new();
    for i in 0..tree_height {
        path.push((leaf_id & (1 << i)) != 0);
    }
    path.reverse(); // 从根到叶子
    path
}


/// Loads an ELF file from the specified path.
pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}
