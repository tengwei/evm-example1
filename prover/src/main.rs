use fibonacci_lib::{load_elf, verify, PublicValuesStruct, UserInfo};
use pico_sdk::{client::DefaultProverClient, init_logger};
use std::{env, fs};
use alloy::hex;
use alloy_sol_types::SolValue;
use fibonacci_lib::types::BlockWitnessCircuit;
use fibonacci_lib::types::custom_base64_decode;

use fibonacci_lib::proof_input::BlockWitnessProofInput;
use alloy_sol_types::{sol};
use serde::Serialize;
use std::borrow::Borrow;
use std::time::Instant;
use tiny_keccak::{Hasher, Keccak};

fn main() {
    let start = Instant::now();

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


    let proof_input: BlockWitnessProofInput = parsed_data.into();


    println!("parsed_data stateRootBefore: 0x{}", hex::encode(&proof_input.state_root_before));
    println!("parsed_data stateRootBefore: 0x{}", hex::encode(&proof_input.state_root_before));
    println!("parsed_data state_root_after: 0x{}", hex::encode(&proof_input.state_root_after));




    verify(&proof_input);

    // let proof_input = BlockWitnessProofInput {
    //     block_height: 123,
    //     user_data_delta_circuit_list: vec![],
    //     commitment: [0u8; 32],
    //     state_root_before: [0u8; 32],
    //     state_root_after: [0u8; 32],
    // };

    //     let json_data = r#"{
    //     "block_height": 123,
    //     "user_data_delta_circuit_list": [],
    //     "commitment": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     "state_root_before": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     "state_root_after": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // }"#;

    // 序列化为 JSON 字符串
    // let serialized = serde_json::to_string(&proof_input).expect("序列化失败");
    // println!("Serialized: {}", serialized);

    // 反序列化为结构体
    // let deserialized: BlockWitnessProofInput = serde_json::from_str(&serialized).expect("反序列化失败");
    // verify(&deserialized);
    //
    // println!("deserialized success");

    // println!("deserialized.block_height: {:?}", deserialized.user_data_delta_circuit_list[1].balances_after[0].balance);


    // let user_info = UserInfo {
    //     addr1: "0x1111111111111111111111111111111111111111".parse().unwrap(),
    //     // balances:
    //     // positions:
    //
    //     balance1: 1.try_into().unwrap(),
    // };
    // 自动支持 ABI 编码
    // let encoded: Vec<u8> = user_info.abi_encode();
    // println!("encoded: 0x{}", hex::encode(&encoded));
    //
    // let mut hasher = Keccak::v256();
    // let mut output = [0u8; 32];
    // hasher.update(&encoded);
    // hasher.finalize(&mut output);
    // println!("Keccak: 0x{}", hex::encode(&output));
    // println!("stdin_builder.write_slice");

    // stdin_builder.write_slice(&serialized.as_ref());
    // stdin_builder.write(&proof_input);

    // stdin_builder.write(&user_info);


    // 打印解析后的数据
    // println!("{:?}", parsed_data);

    // Set up input
    // let n = 10u32;
    // stdin_builder.write(&deserialized);


    // stdin_builder.write(&n);

    stdin_builder.write(&proof_input);


    // Set up output path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    // let output_path = current_dir.join(format!("{}{}", "../contracts/test_data/", proof_input.block_height));
    let output_path = current_dir.join( "../contracts/test_data/");


    fs::create_dir_all(&output_path).expect("Failed to create directory");


    // Set up groth16 verifier and generate pico proof
    // The first parameter `need_setup = true` ensures the Groth16 verifier is set up,
    // but this setup is required only once.
    client
        .prove_evm(stdin_builder, false, output_path.clone(), "kb")
        .expect("Failed to generate evm proof");

    // Generate proof
    // let proof = client
    //     .prove_fast(stdin_builder)
    //     .expect("Failed to generate proof");

    // client
    //     .emulate(stdin_builder);

    // Decodes public values from the proof's public value stream.
    // let public_buffer = proof.pv_stream.unwrap();
    //
    // // Deserialize public_buffer into PublicValuesStruct
    // let public_values: PublicValuesStruct =
    //     bincode::deserialize(&public_buffer).expect("Failed to deserialize");

    // let public_values: PublicValuesStruct =
    //     serde_json::from_slice(&public_buffer).expect("Failed to deserialize");

    // let deserialized: Example = serde_json::from_str(json_data).expect("反序列化失败");

    // // Verify the public values
    // verify_public_values(&public_values);

    let duration = start.elapsed();

    println!("程序运行耗时: {:?}", duration);


}

/// Verifies that the computed Fibonacci values match the public values.
fn verify_public_values(public_values: &PublicValuesStruct) {
    println!(
        "Public value depositSuccessHeight: {:?}, blockHeight: {:?}, stateRootBefore: {:?},stateRootAfter: {:?}", public_values.depositSuccessHeight, public_values.blockHeight, public_values.stateRootBefore, public_values.stateRootAfter
    );

    // Compute Fibonacci values locally
    // let (result_a, result_b) = verify();

    // Assert that the computed values match the public values
    // assert_eq!(result_a, public_values.a, "Mismatch in value 'a'");
    // assert_eq!(result_b, public_values.b, "Mismatch in value 'b'");
}


#[cfg(test)]
mod tests {
    use alloy_primitives::{Address, FixedBytes, Signed, I256};
    use base64::Engine;
    use base64::engine::general_purpose;
    use serde::Deserialize;
    use super::*;
    use serde_json;
    use fibonacci_lib::proof_input::{BalanceABI, PositionABI};
    use fibonacci_lib::UserInfo1;

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


    fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
        general_purpose::STANDARD
            .decode(input)
            .map_err(|e| format!("Base64 解码失败: {}", e))
    }

    #[test]
    fn test_base64() {
        let base64_string = "VYrZAbnvvGnRk4iu5IcIzaFqFkt5kRT0n7Zj//fYU3s="; // 示例 Base64 字符串
        match decode_base64(base64_string) {
            Ok(decoded) => {
                println!("解码结果: {:?}", decoded);
                println!("Keccak: 0x{}", hex::encode(&decoded));
            }
            Err(err) => println!("{}", err),
        }
    }

    #[test]
    fn test_serde_bytes() {
        #[derive(Serialize, Deserialize, Debug)]
        struct Example {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }
        // let example = Example {
        //     data: vec![1, 2, 3, 4, 5],
        // };

        let json_data = r#"{
    "data": "VYrZAbnvvGnRk4iu5IcIzaFqFkt5kRT0n7Zj//fYU3s="}"#;

        // 序列化为 JSON
        // let serialized = serde_json::to_string(&json_data).unwrap();
        // println!("Serialized: {}", serialized);

        // 反序列化为结构体
        let deserialized: Example = serde_json::from_str(&json_data).unwrap();
        println!("Deserialized: {:?}", hex::encode(&deserialized.data));
    }

    #[test]
    fn test_custom_base64_decode() {
        #[derive(Serialize, Deserialize, Debug)]
        struct Example {
            #[serde(deserialize_with = "custom_base64_decode")]
            data: Vec<u8>,
        }
        let json_data = r#"{
        "data": "VYrZAbnvvGnRk4iu5IcIzaFqFkt5kRT0n7Zj//fYU3s="
    }"#;

        let deserialized: Example = serde_json::from_str(json_data).expect("反序列化失败");

        println!("解码后的数据: {:?}", deserialized.data);
        println!("十六进制表示: 0x{}", alloy::hex::encode(&deserialized.data));
    }

    #[test]
    fn test_keccak_hex_data() {
        // 输入的十六进制数据
        let hex_data = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000020558ad901b9efbc69d19388aee48708cda16a164b799114f49fb663fff7d8537b0000000000000000000000000000000000000000000000000000000000000020fe009b1873f0e65fa7bf8663b82a3ba4ed5429908f379d8403fb4059f3ebbe60";

        // 将十六进制字符串转换为字节数组
        let data = hex::decode(hex_data).expect("十六进制解码失败");

        // 初始化 Keccak-256 哈希计算器
        let mut hasher = Keccak::v256();
        let mut output = [0u8; 32];

        // 更新哈希计算器并计算哈希值
        hasher.update(&data);
        hasher.finalize(&mut output);

        // 打印结果
        println!("Keccak-256 哈希值: 0x{}", hex::encode(output));
    }

    #[test]
    fn test_user_info_abi_encoding1() {
        // let user_info = UserInfo1 {
        //     addr1: "0x1111111111111111111111111111111111111111".parse().unwrap(),
        //     balances: vec![],
        //     positions: vec![],
        // };

        let user_info = UserInfo1 {
            addr1: "0x1111111111111111111111111111111111111111".parse::<Address>().unwrap(),
            balances: vec![
                BalanceABI {
                    assetName: FixedBytes::from([0u8; 32]),
                    balance: Signed::<256, 4>::ZERO,
                    maxWithdrawAmount: Signed::<256, 4>::ZERO,
                },
            ],
            positions: vec![
                PositionABI {
                    symbolName: FixedBytes::from([0u8; 32]),
                    positionItems: vec![],
                },
            ],
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