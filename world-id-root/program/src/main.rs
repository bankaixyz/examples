//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use bankai_types::ProofBundle;
use bankai_verify::verify_batch_proof;

pub fn main() {
    let bundle = sp1_zkvm::io::read::<ProofBundle>();

    let result = verify_batch_proof(bundle).expect("Failed to verify batch proof");
    let slot_value: [u8; 32] = result.evm.storage_slot[0].slots[0].1.to_be_bytes();

    sp1_zkvm::io::commit_slice(slot_value.as_slice());
}
