#![no_main]

pico_sdk::entrypoint!(main);
use alloy_sol_types::SolValue;
use fibonacci_lib::{PublicValuesStruct, fibonacci};
use pico_sdk::io::{commit_bytes, read_as};
use fibonacci_lib::proof_input::BlockWitnessProofInput;
use fibonacci_lib::types::BlockWitnessCircuit;

pub fn main() {
    // Read inputs `n` from the environment

    let proof_input: BlockWitnessProofInput = pico_sdk::io::read_as();

    let n: u32 = 0;

    let a: u32 = 0;
    let b: u32 = 1;

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
