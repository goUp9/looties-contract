Sure! Here's how the above information looks in a readme.md file:

Diff
Copy
Insert
New
# looties-contract

This is a Rust library named "looties-contract". It was created with Anchor and is currently at version 0.1.0.

## Features
The following features are included in this crate:
* No entrypoint
* No idl
* No log ix name
* cpi (requires no-entrypoint)

## Dependencies
This project has the following dependencies:
* `anchor-lang` version 0.29.0, with feature "init-if-needed"
* `anchor-spl` version 0.29.0, with feature "metadata"
* `sha2` version 0.10.8
* `solana-program` version 1.17.7
* `spl-associated-token-account` version 2.2.0

## Usage
To use this package in your Rust project, add the following to your Cargo.toml file:

```toml
[dependencies]
looties-contract = "0.1.0"