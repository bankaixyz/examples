#![no_main]
sp1_zkvm::entrypoint!(main);

use bankai_types::ProofBundle;
use bankai_verify::verify_batch_proof;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicValues {
    pub block_number: u64,
    pub address: [u8; 20],
    pub balance: [u8; 32],
}

pub fn main() {
    let bundle = sp1_zkvm::io::read::<ProofBundle>();

    let result = verify_batch_proof(bundle).expect("failed to verify batch proof");
    let account = &result.op_stack.account[0];

    let mut address = [0u8; 20];
    address.copy_from_slice(account.address.as_slice());

    let public_values = PublicValues {
        block_number: account.block.block_number,
        address,
        balance: account.account.balance.to_be_bytes(),
    };

    sp1_zkvm::io::commit(&public_values);
}
