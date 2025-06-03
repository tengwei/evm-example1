#![feature(portable_simd)]

pub mod types;
pub mod proof_input;

use std::fs;
use alloy::hex;
use alloy_primitives::{address, Address, FixedBytes, I256};
use alloy_sol_types::{sol, SolValue};
use serde::{Deserialize, Deserializer, Serialize};
use tiny_keccak::{Hasher, Keccak};
use crate::proof_input::{BalanceABI, BlockWitnessProofInput, PositionABI, UserDataABI};

const ACCOUNT_MERKLE_LEVELS: usize = 24;
// sol! {
//     /// The public values encoded as a struct that can be easily deserialized inside Solidity.
//     struct PublicValuesStruct {
//         uint32 n;
//         uint32 a;
//         uint32 b;
//     }
// }

sol! {
    //     /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    #[derive(Debug, Serialize, Deserialize)]
    struct PublicValuesStruct {
        uint64 depositSuccessHeight;
        uint64 blockHeight;
        bytes32 stateRootBefore;
        bytes32 stateRootAfter;
    }
}

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct UserInfo {
        address addr1;
        uint256 balance1;
    }
}

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct UserInfo1 {
        address addr1;
        BalanceABI[] balances;
        PositionABI[] positions;
    }
}

// fn deserialize_i256<'de, D>(deserializer: D) -> Result<I256, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     s.parse::<I256>().map_err(serde::de::Error::custom)
// }

pub fn verify(block_witness: &BlockWitnessProofInput) -> () {
    println!("verify block height: {}", block_witness.block_height);

    let size = block_witness.user_data_delta_circuit_list.len();
    // let size = 1;

    // println!("block_witness.commitment: {}", hex::encode(block_witness.commitment));


    assert!(compare(block_witness.commitment, keccak256_for_commitment(block_witness.deposit_success_height, block_witness.block_height, block_witness.state_root_before, block_witness.state_root_after)));

    if size == 0 {
        //state_root_before must be equal state_root_after
        assert_eq!(block_witness.state_root_before, block_witness.state_root_after, "Mismatch State Root!");
    } else {
        assert_eq!(block_witness.state_root_before, block_witness.user_data_delta_circuit_list[0].state_root_before, "Mismatch State Root!");
        for user_data in block_witness.user_data_delta_circuit_list.iter() {
            // println!("Account ID: {}", user_data.account_id);
            // println!("Address Before: {:?}", user_data.address_before);
            // println!("Address After: {:?}", user_data.address_after);
            // println!("State Root Before: {:?}", user_data.state_root_before);
            // println!("State Root After: {:?}", user_data.state_root_after);

            let hash_before = generate_leaf_hash(user_data.abi_encode_before);
            let is_valid_before = verify_sparse_merkle_root(user_data.account_id, hash_before, user_data.merkle_proofs_before, user_data.state_root_before);
            // assert!(is_valid_before, "Mismatch State Root!");

            let hash_after = generate_leaf_hash(user_data.abi_encode_after);
            let is_valid_after = verify_sparse_merkle_root(user_data.account_id, hash_after, user_data.merkle_proofs_after, user_data.state_root_after);
            // assert!(is_valid_after, "Mismatch State Root!");
            println!("verify success Account ID: {}", user_data.account_id);

            //todo
        }
        assert_eq!(block_witness.state_root_after, block_witness.user_data_delta_circuit_list[size - 1].state_root_after, "Mismatch State Root!");
    }
    println!("verify success");
}

fn keccak256_for_commitment(deposit_success_height: u64, block_height: u64, state_root_before: [u8; 32], state_root_after: [u8; 32]) -> [u8; 32] {
    // ABI encode
    let encoded: Vec<u8> = PublicValuesStruct {
        depositSuccessHeight: deposit_success_height,
        blockHeight: block_height,
        stateRootBefore: FixedBytes::from(state_root_before),
        stateRootAfter: FixedBytes::from(state_root_after),
    }.abi_encode();

    // println!("commitment depositSuccessHeight: {}", deposit_success_height);
    // println!("commitment block_height: {}", block_height);
    // println!("commitment stateRootBefore: 0x{}", hex::encode(&state_root_before));
    // println!("commitment state_root_after: 0x{}", hex::encode(&state_root_after));


    // println!("commitment encoded: 0x{}", hex::encode(&encoded));

    // Keccak256 hash
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(&encoded);
    hasher.finalize(&mut output);
    // println!("commitment: {}", hex::encode(&output));

    output
}

fn vec_to_bytes32(vec: Vec<u8>) -> [u8; 32] {
    let mut bytes32 = [0u8; 32];
    let len = vec.len().min(32); // Make sure it does not exceed 32 bytes
    bytes32[..len].copy_from_slice(&vec[..len]);
    bytes32
}

fn vec_to_array(vec: Vec<Vec<u8>>) -> [[u8; 32]; ACCOUNT_MERKLE_LEVELS] {
    vec.into_iter()
        .map(|v| {
            let mut arr = [0u8; 32];
            let len = v.len().min(32); // 确保不超过 32 字节
            arr[..len].copy_from_slice(&v[..len]);
            arr
        })
        .collect::<Vec<[u8; 32]>>()
        .try_into().unwrap()
}

fn compare(vec: [u8; 32], arr: [u8; 32]) -> bool {
    // Check vec length to avoid panic
    if vec.len() != 32 {
        return false;
    }
    // Compare contents (as_slice converts to &[u8], which automatically supports comparison with [u8; 32])
    arr == vec
}

pub fn generate_leaf_hash( encoded: [u8; 32]) -> [u8; 32] {
    // ABI encode
    // let encoded: Vec<u8> = UserDataABI { address, balances, positions }.abi_encode();
    // println!("generate_leaf_hash userAddress: 0x{}", hex::encode(&address));

    // println!("generate_leaf_hash encoded: 0x{}", hex::encode(&encoded));

    // Keccak256 hash
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(&encoded);
    hasher.finalize(&mut output);
    // println!("generate_leaf_hashr: {:?}", hex::encode(&output));

    output
}

fn verify_sparse_merkle_root(
    account_id: i64,
    leaf_hash: [u8; 32],
    merkle_proofs: [[u8; 32]; ACCOUNT_MERKLE_LEVELS],
    state_root: [u8; 32],
) -> bool {
    let mut current_hash = leaf_hash;
    //todo
    let path = leaf_id_to_path(account_id);
    // println!("path: {:?}", path);


    // for merkle_proof in merkle_proofs {
    //     println!("verify_sparse_merkle_root encoded: 0x{}", hex::encode(&merkle_proof));
    // }

    for (proof_hash, is_right) in merkle_proofs.iter().zip(path.iter()) {
        current_hash = if *is_right {
            hash_node(proof_hash, &current_hash)
        } else {
            hash_node(&current_hash, proof_hash)
        };
    }

    // println!("verify_sparse_merkle_root current_hash {:?},state_root {:?}", current_hash, state_root);

    current_hash == state_root
}

fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    // println!("left encoded: 0x{}", hex::encode(&left));
    // println!("right encoded: 0x{}", hex::encode(&right));

    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(left);
    hasher.update(right);
    hasher.finalize(&mut output);
    output
}

fn leaf_id_to_path(key: i64) -> Vec<bool> {
    let mut path = Vec::with_capacity(ACCOUNT_MERKLE_LEVELS);
    let mut depth: u8 = 4;
    for i in 0..(ACCOUNT_MERKLE_LEVELS / 4) as usize {
        let path_id = key >> (ACCOUNT_MERKLE_LEVELS as usize - (i + 1) * 4);
        let nibble = path_id & 0x000000000000000f;

        if i > 0 {
            // ignore the root node
            path.push(((path_id / 16 % 2) as i32) != 0);
        }

        let mut index = 0;
        for j in 0..3 {
            // nibble / 8
            // nibble / 4
            // nibble / 2
            let inc = (nibble / (1 << (3 - j))) as i32;
            path.push((inc % 2) != 0);
            index += 1 << (j + 1);
        }

        depth += 4;
    }
    path.push(((key % 2) as i32) != 0);
    path.reverse();
    path
}

// fn leaf_id_to_path(leaf_id: i64) -> Vec<bool> {
//     let mut path = Vec::new();
//     for i in 0..ACCOUNT_MERKLE_LEVELS {
//         path.push((leaf_id & (1 << i)) != 0);
//     }
//     path.reverse(); // 从根到叶子
//
//
//     path
// }


/// Loads an ELF file from the specified path.
pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}
