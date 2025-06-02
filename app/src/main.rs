#![no_main]

pico_sdk::entrypoint!(main);

use alloy_sol_types::private::FixedBytes;
use alloy_sol_types::SolValue;
use fibonacci_lib::proof_input::BlockWitnessProofInput;
use fibonacci_lib::{verify, PublicValuesStruct, UserInfo};
use pico_sdk::io::{commit, commit_bytes, read_as, read_vec};

pub fn main() {
    // Read inputs `n` from the environment


    // let user_info: UserInfo = read_as();
    // println!("addr1: {}", user_info.addr1);
    // println!("balance1: {}", user_info.balance1);

    println!("proof_input_json start");

    let  proof_input: BlockWitnessProofInput = read_as();

    // let proof_input_json = read_vec();

    println!("proof_input_json end");


    // let proof_input: BlockWitnessProofInput = serde_json::from_slice(&proof_input_json).expect("反序列化失败");
    // println!("test111111: {}", user_info.balance1);
    println!("verify start");

    // verify(&proof_input);


    // let n: u32 = read_as();

    let n: u32 = 0;


    // Compute Fibonacci values starting from `a` and `b`
    // let (a_result, b_result) = fibonacci(a, b, n);

    // Encode the result into ABI format
    let result = PublicValuesStruct {
        depositSuccessHeight: proof_input.deposit_success_height,
        blockHeight: proof_input.block_height,
        stateRootBefore: FixedBytes::from(proof_input.state_root_before),
        stateRootAfter: FixedBytes::from(proof_input.state_root_after),
    };
    // let encoded_bytes = result.abi_encode();

    // commit_bytes(&encoded_bytes);

    commit(&result);

}
