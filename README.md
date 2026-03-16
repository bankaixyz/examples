# Bankai Examples

Example projects showing how to build with the [Bankai SDK](https://github.com/bankaixyz/bankai-sdk).

Bankai is a stateless Ethereum light client that compresses its trusted state into recursive zk proofs. The SDK lets you read any on-chain data — storage slots, accounts, transactions, receipts — with full cryptographic verification. The verified data can be used directly in your application or fed into a zkVM program to produce proofs that are verifiable on other chains.

These examples demonstrate end-to-end patterns for common use cases.

## Examples

| Example | Description |
|---------|-------------|
| [world-id-root](./world-id-root) | Read the World ID identity root from Ethereum and produce a Groth16 proof that is verifiable on any chain — no bridges, relays, or destination-side light clients required |
