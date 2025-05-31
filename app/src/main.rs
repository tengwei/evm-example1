#![no_main]

pico_sdk::entrypoint!(main);
use alloy_sol_types::SolValue;
use fibonacci_lib::proof_input::BlockWitnessProofInput;
use fibonacci_lib::{verify, PublicValuesStruct};
use pico_sdk::io::{commit_bytes, read_as};

pub fn main() {
    // Read inputs `n` from the environment

    let proof_input: BlockWitnessProofInput = read_as();

    // verify(proof_input);

    let n: u32 = 0;


    // Compute Fibonacci values starting from `a` and `b`
    // let (a_result, b_result) = fibonacci(a, b, n);

    // Encode the result into ABI format
    let result = PublicValuesStruct {
        n,
        a: 0,
        b: 0,
    };
    let encoded_bytes = result.abi_encode();

    commit_bytes(&encoded_bytes);
}
