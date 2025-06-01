use fibonacci_lib::{load_elf, verify, UserInfo};
use pico_sdk::{client::DefaultProverClient, init_logger};
use std::{env, fs};
use alloy::hex;
use alloy_sol_types::SolValue;
use fibonacci_lib::types::BlockWitnessCircuit;
use fibonacci_lib::proof_input::BlockWitnessProofInput;
use alloy_sol_types::{sol};
use serde::Serialize;
use std::borrow::Borrow;
use tiny_keccak::{Hasher, Keccak};

fn main() {
    // Initialize logger
    init_logger();

    // Load the ELF file
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");

    // Initialize the prover client
    let client = DefaultProverClient::new(&elf);
    // Initialize new stdin
    let mut stdin_builder = client.new_stdin_builder();

    // let json_data = r#"{
    //     "stateRootBefore": "c2Rmc2RmZHM=",
    //     "stateRootAfter": "c2Rmc2RmZHM="
    // }"#;
    //
    // let result: Example = serde_json::from_str(json_data).unwrap();
    // println!("{:?}", result);

    // 打印解码后的字节数组
    // println!("stateRootBefore: {:?}", result.state_root_before);
    // println!("stateRootAfter: {:?}", result.state_root_after);


    // 读取JSON文件内容
    let file_content = fs::read_to_string("../witness/1/block_witness_circuit.json").expect("无法读取文件");

    // 解析JSON内容为结构体
    let parsed_data: BlockWitnessCircuit =
        serde_json::from_str(&file_content).expect("JSON解析失败");

    println!("parsed_data stateRootBefore: 0x{}", hex::encode(&parsed_data.state_root_before));
    println!("parsed_data state_root_after: 0x{}", hex::encode(&parsed_data.state_root_after));


    let  proof_input: BlockWitnessProofInput = parsed_data.into();

    verify(&proof_input);

    // let proof_input = BlockWitnessProofInput {
    //     block_height: 123,
    //     user_data_delta_circuit_list: vec![],
    //     commitment: [0u8; 32],
    //     state_root_before: [0u8; 32],
    //     state_root_after: [0u8; 32],
    // };

    let json_data = r#"{
    "block_height": 123,
    "user_data_delta_circuit_list": [],
    "commitment": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    "state_root_before": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    "state_root_after": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}"#;

    // 序列化为 JSON 字符串
    let serialized = serde_json::to_string(&proof_input).expect("序列化失败");
    // println!("Serialized: {}", serialized);

    // 反序列化为结构体
    let deserialized: BlockWitnessProofInput = serde_json::from_str(&*serialized).expect("反序列化失败");
    println!("deserialized.block_height: {:?}", deserialized.user_data_delta_circuit_list[1].balances_after[0].balance);





    let user_info = UserInfo {
        addr1: "0x1111111111111111111111111111111111111111".parse().unwrap(),
        // balances:
        // positions:

        balance1: 1.try_into().unwrap(),
    };
    // 自动支持 ABI 编码
    let encoded: Vec<u8> = user_info.abi_encode();
    println!("encoded: 0x{}", hex::encode(&encoded));

    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(&encoded);
    hasher.finalize(&mut output);
    println!("Keccak: 0x{}", hex::encode(&output));

    stdin_builder.write(&user_info);
    // 打印解析后的数据
    // println!("{:?}", parsed_data);

    // Set up input
    let n = 10u32;
    stdin_builder.write(&deserialized);


    stdin_builder.write(&n);

    // stdin_builder.write(&proof_input);


    // Set up output path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let output_path = current_dir.join("../contracts/test_data");

    // Set up groth16 verifier and generate pico proof
    // The first parameter `need_setup = true` ensures the Groth16 verifier is set up,
    // but this setup is required only once.
    client
        .prove_evm(stdin_builder, false, output_path.clone(), "kb")
        .expect("Failed to generate evm proof");
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_block_witness_proof_input_serialization() {
        let proof_input = BlockWitnessProofInput {
            block_height: 123,
            user_data_delta_circuit_list: vec![],
            commitment: [0u8; 32],
            state_root_before: [0u8; 32],
            state_root_after: [0u8; 32],
            deposit_success_height: 0,
        };

        // 测试序列化
        let serialized = serde_json::to_string(&proof_input).expect("序列化失败");
        assert!(serialized.contains("\"block_height\":123"));

        // 测试反序列化
        let deserialized: BlockWitnessProofInput =
            serde_json::from_str(&serialized).expect("反序列化失败");
        assert_eq!(deserialized.block_height, proof_input.block_height);
        assert_eq!(deserialized.commitment, proof_input.commitment);
    }

    #[test]
    fn test_user_info_abi_encoding() {
        let user_info = UserInfo {
            addr1: "0x1111111111111111111111111111111111111111".parse().unwrap(),
            balance1: 1.try_into().unwrap(),
        };

        // 测试 ABI 编码
        let encoded: Vec<u8> = user_info.abi_encode();
        assert!(!encoded.is_empty());
        println!("encoded: 0x{}", hex::encode(&encoded));


        // 测试 Keccak 哈希计算
        let mut hasher = Keccak::v256();
        let mut output = [0u8; 32];
        hasher.update(&encoded);
        hasher.finalize(&mut output);
        println!("Keccak: 0x{}", hex::encode(&output));

        assert_eq!(output.len(), 32);
    }
}