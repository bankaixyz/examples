# Trustless Base account balance proof with Bankai

This example asks Bankai for the latest Base height, proves the balance of
`0x4200000000000000000000000000000000000016` at that height, and then verifies the
result inside an SP1 zkVM program.

The host script does two things before entering the zkVM:

1. Resolve the latest Bankai block number.
2. Resolve the Base height for that exact Bankai block and request a proof bundle
   for the target account at that Base height.

The guest program verifies the proof bundle with `bankai-verify` and commits the
verified Base block number, address, and balance as public output.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)
- A Base RPC URL exposed as `BASE_RPC`

## Running

Execute the zkVM program locally without proof generation:

```sh
cd script
cargo run --release -- --execute
```

Generate a Groth16 proof:

```sh
cd script
cargo run --release -- --prove
```

> [!NOTE]
> Proof generation requires a minimum of 16 GB RAM. For production workloads,
> use the
> [Succinct Prover Network](https://docs.succinct.xyz/docs/next/sp1/prover-network/quickstart)
> by setting `NETWORK_PRIVATE_KEY` in a `.env` file.

## Environment

Required:

- `BASE_RPC`: RPC URL for Base. The SDK uses this to build the account proof for
  the verified Base block.

Optional:

- `NETWORK_PRIVATE_KEY`: required only for `--prove`

## Version constraints

This example follows the same SP1 and toolchain constraints as the other example
in this repo:

- `resolver = "3"`
- `nightly-2025-07-14`
- `sp1-sdk = "=5.2.2"`
- checked-in `Cargo.lock` with `smol_str = 0.3.2`
- the required `ethereum_hashing` patch
