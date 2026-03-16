use std::collections::BTreeMap;

use alloy_primitives::{hex::FromHex, Address};
use bankai_sdk::{Bankai, HashingFunction, Network};
use bankai_types::api::ethereum::BankaiBlockFilterDto;
use clap::Parser;
use serde::{Deserialize, Serialize};
use sp1_sdk::network::NetworkMode;
use sp1_sdk::{include_elf, Prover, SP1Stdin};

const BASE_BALANCE_ELF: &[u8] = include_elf!("base-balance");
const TARGET_ADDRESS: &str = "0x4200000000000000000000000000000000000016";
const BASE_CHAIN: &str = "base";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct PublicValues {
    block_number: u64,
    address: [u8; 20],
    balance: [u8; 32],
}

#[tokio::main]
async fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("error: specify exactly one of --execute or --prove");
        std::process::exit(1);
    }

    let base_rpc = std::env::var("BASE_RPC").expect("BASE_RPC must be set");
    let target = Address::from_hex(TARGET_ADDRESS).expect("invalid target address");

    let bankai = Bankai::new(
        Network::Sepolia,
        None,
        None,
        Some(BTreeMap::from([(BASE_CHAIN.to_string(), base_rpc)])),
    );

    let latest_bankai_block = bankai.api.blocks().latest_number().await.unwrap();
    let filter = BankaiBlockFilterDto::with_bankai_block_number(latest_bankai_block);
    let latest_base_height = bankai
        .api
        .op_stack()
        .height(BASE_CHAIN, &filter)
        .await
        .unwrap();

    println!("Bankai block: {latest_bankai_block}");
    println!(
        "Latest Base height at that Bankai block: {}",
        latest_base_height.height
    );

    let bundle = bankai
        .init_batch(Some(latest_bankai_block), HashingFunction::Keccak)
        .await
        .unwrap()
        .op_stack_account(BASE_CHAIN, latest_base_height.height, target)
        .execute()
        .await
        .unwrap();

    let mut stdin = SP1Stdin::new();
    stdin.write(&bundle);

    if args.execute {
        let client = sp1_sdk::ProverClient::builder().mock().build();
        let (mut output, report) = client.execute(BASE_BALANCE_ELF, &stdin).run().unwrap();
        let public_values = output.read::<PublicValues>();

        println!("Program executed successfully.");
        println!("Verified Base block: {}", public_values.block_number);
        println!(
            "Verified address: 0x{}",
            alloy_primitives::hex::encode(public_values.address)
        );
        println!(
            "Verified balance: {}",
            alloy_primitives::U256::from_be_bytes(public_values.balance)
        );
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        let private_key =
            std::env::var("NETWORK_PRIVATE_KEY").expect("NETWORK_PRIVATE_KEY must be set");

        let client = sp1_sdk::ProverClient::builder()
            .network_for(NetworkMode::Mainnet)
            .private_key(&private_key)
            .build();

        let (pk, vk) = client.setup(BASE_BALANCE_ELF);
        let mut proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");

        let public_values = proof.public_values.read::<PublicValues>();

        println!("Successfully generated proof!");
        println!("Verified Base block: {}", public_values.block_number);
        println!(
            "Verified address: 0x{}",
            alloy_primitives::hex::encode(public_values.address)
        );
        println!(
            "Verified balance: {}",
            alloy_primitives::U256::from_be_bytes(public_values.balance)
        );

        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
