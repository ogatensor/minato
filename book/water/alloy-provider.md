## Navigation

- [Interface with an Ethereum blockchain](#interface-with-an-ethereum-blockchain)
- [Feature flags](#feature-flags)

## Interface with an Ethereum blockchain

This crate contains the `Provider` trait, which exposes Ethereum JSON-RPC methods. Providers in alloy are similar to [ethers.js](https://docs.ethers.org/v6/) providers. They manage an `RpcClient` and allow other parts of the program to easily make RPC calls.

Unlike an [ethers.js](https://docs.ethers.org/v6/) Provider, an alloy Provider is network-aware. It is parameterized with a `Network` from [alloy-networks](https://github.com/alloy-rs/alloy/tree/main/crates/networks). This allows the Provider to expose a consistent interface to the rest of the program, while adjusting request and response types to match the underlying blockchain.

Providers can be composed via stacking. For example, a `Provider` that tracks the nonce for a given address can be stacked onto a `Provider` that signs transactions to create a `Provider` that can send signed transactions with correct nonces.

The `ProviderBuilder` struct can quickly create a stacked provider, similar to [tower::ServiceBuilder](https://docs.rs/tower/latest/tower/struct.ServiceBuilder.html).

### Key Data Points

| Feature | Description |
| --- | --- |
| Network-aware | Parameterized with a `Network` from [alloy-networks](https://github.com/alloy-rs/alloy/tree/main/crates/networks) |
| Composable | Providers can be stacked to create new providers with additional functionality |
| JSON-RPC methods | Exposes Ethereum JSON-RPC methods |
| RpcClient management | Manages an `RpcClient` for making RPC calls |
| Consistent interface | Exposes a consistent interface to the rest of the program |
| Adjustable request/response types | Adjusts request and response types to match the underlying blockchain |

### Usage Example

```rust
use alloy::provider::{Provider, ProviderBuilder};
use alloy::networks::mainnet::Mainnet;

// Create a provider for the Ethereum mainnet
let provider = ProviderBuilder::new(Mainnet)
    // Add a nonce tracker for the address 0x123456...
    .with_nonce_tracker("0x123456...")
    // Add a transaction signer with the private key 0x123456...
    .with_transaction_signer("0x123456...")
    .build();

// Use the provider to send a transaction
let tx = provider.send_transaction(tx_data).await?;
```

## Feature flags

- `pubsub` - Enable support for subscription methods.
- `ws` - Enable WebSocket support. Implictly enables `pubsub`.
- `ipc` - Enable IPC support. Implictly enables `pubsub`.
