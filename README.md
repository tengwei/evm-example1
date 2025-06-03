## Foundry

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

-   **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
-   **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
-   **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
-   **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Documentation

https://book.getfoundry.sh/

## Usage

### Install
```shell
yarn install
forge install foundry-rs/forge-std --no-commit
forge install OpenZeppelin/openzeppelin-contracts-upgradeable@v5.0.2 --no-commit
forge install OpenZeppelin/openzeppelin-foundry-upgrades@v0.3.6 --no-commit
```


### Build

```shell
$ forge clean && forge build --via-ir
```

### Test

```shell
$ forge clean && forge test -vvv --via-ir
```

# prover
cd prover

RUST_LOG=debug RUST_LOGGER=forest cargo run --release
RUST_LOG=info cargo run --release

# app
cd app
RUST_LOG=info cargo pico build   