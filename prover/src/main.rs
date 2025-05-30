use fibonacci_lib::{load_elf};
use pico_sdk::{client::DefaultProverClient, init_logger};
use std::{env, fs};
use alloy_sol_types::private::primitives::hex::hex;
use fibonacci_lib::types::BlockWitnessCircuit;
use fibonacci_lib::types::Example;


fn main() {
    // Initialize logger
    init_logger();

    // Load the ELF file
    let elf = load_elf("../elf/riscv32im-pico-zkvm-elf");

    // Initialize the prover client
    let client = DefaultProverClient::new(&elf);
    // Initialize new stdin
    let mut stdin_builder = client.new_stdin_builder();

    let json_data = r#"{
        "stateRootBefore": "c2Rmc2RmZHM=",
        "stateRootAfter": "c2Rmc2RmZHM="
    }"#;

    let result: Example = serde_json::from_str(json_data).unwrap();
    println!("{:?}", result);

    // 打印解码后的字节数组
    println!("stateRootBefore: {:?}", result.state_root_before);
    println!("stateRootAfter: {:?}", result.state_root_after);


    // 读取JSON文件内容
    let file_content = fs::read_to_string("../witness/1/block_witness_circuit.json").expect("无法读取文件");

    // 解析JSON内容为结构体
    let parsed_data: BlockWitnessCircuit =
        serde_json::from_str(&file_content).expect("JSON解析失败");

    // let user_data = UserData {
    //     userAddress: "0x1111111111111111111111111111111111111111".parse().unwrap(),
    //     // balances:
    //     // positions:
    //
    //     balances: vec![],
    //     positions: vec![],
    // };
    // // 自动支持 ABI 编码
    // let encoded: Vec<u8> = user_data.abi_encode();
    // println!("encoded: 0x{}", hex::encode(&encoded));




    // 打印解析后的数据
    println!("{:?}", parsed_data);

    // Set up input
    let n = 10u32;
    stdin_builder.write(&n);

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