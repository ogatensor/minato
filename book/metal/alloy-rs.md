# [[Alloy]]

## Navigation
- [[#Overview|Overview]]
- [[#Supported Rust Versions|Supported Rust Versions]]
- [[#Contributing|Contributing]]
- [[#License|License]]

## Overview
Alloy is a Rust library for interacting with Ethereum blockchains. It is a rewrite of the [ethers-rs](https://github.com/gakonst/ethers-rs) library from the ground up, with new features, high performance, and excellent documentation.

The repository contains the following crates:

- [[alloy|alloy]]: Meta-crate for the entire project, including `alloy-core`
- [[alloy-consensus|alloy-consensus]]: Ethereum consensus interface
- [[alloy-contract|alloy-contract]]: Interact with on-chain contracts
- [[alloy-eips|alloy-eips]]: Ethereum Improvement Proposal (EIP) implementations
- [[alloy-genesis|alloy-genesis]]: Ethereum genesis file definitions
- [[alloy-json-rpc|alloy-json-rpc]]: Core data types for JSON-RPC 2.0 clients
- [[alloy-network|alloy-network]]: Network abstraction for RPC types
- [[alloy-node-bindings|alloy-node-bindings]]: Ethereum execution-layer client bindings
- [[alloy-provider|alloy-provider]]: Interface with an Ethereum blockchain
- [[alloy-pubsub|alloy-pubsub]]: Ethereum JSON-RPC publish-subscribe tower service and type definitions
- [[alloy-rpc-client|alloy-rpc-client]]: Low-level Ethereum JSON-RPC client implementation
- [[alloy-rpc-types|alloy-rpc-types]]: Ethereum JSON-RPC types
- [[alloy-rpc-types-engine|alloy-rpc-types-engine]]: Ethereum execution-consensus layer (engine) API RPC types
- [[alloy-rpc-types-trace|alloy-rpc-types-trace]]: Ethereum RPC trace types
- [[alloy-signer|alloy-signer]]: Ethereum signer abstraction
- [[alloy-signer-aws|alloy-signer-aws]]: AWS KMS signer implementation
- [[alloy-signer-gcp|alloy-signer-gcp]]: GCP KMS signer implementation
- [[alloy-signer-ledger|alloy-signer-ledger]]: Ledger signer implementation
- [[alloy-signer-trezor|alloy-signer-trezor]]: Trezor signer implementation
- [[alloy-signer-wallet|alloy-signer-wallet]]: Local wallet (Keystore/Mnemonic/Yubihsm) signer implementation
- [[alloy-transport|alloy-transport]]: Low-level Ethereum JSON-RPC transport abstraction
- [[alloy-transport-http|alloy-transport-http]]: HTTP transport implementation
- [[alloy-transport-ipc|alloy-transport-ipc]]: IPC transport implementation
- [[alloy-transport-ws|alloy-transport-ws]]: WS transport implementation

## Supported Rust Versions
Alloy will keep a rolling MSRV (minimum supported rust version) policy of at least 6 months. The current MSRV is 1.76.

## Contributing
Contributions are welcome! Please follow the [contributing guide](https://github.com/alloy-rs/alloy/blob/main/CONTRIBUTING.md) to get involved in the Alloy project.

## License
Alloy is licensed under either the Apache License, Version 2.0 or the MIT license.
