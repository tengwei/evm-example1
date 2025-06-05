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

# 压测
CPU这边有些参数可以适配具体的机器环境，会影响cpu这边的性能和内存占用量。不过production performance还是以gpu为主

文档：https://pico-docs.brevis.network/writing-apps/features

export CHUNK_SIZE=4194304           # 切片大小
export CHUNK_BATCH_SIZE=32        # 内存中排队的切片数量
export RUST_LOG=info                        # 日志级别
export RUSTFLAGS="-C target-cpu=native -C target-feature=+avx512f,+avx512ifma,+avx512vl"    # 有的机器提供avx512指令集支持
export JEMALLOC_SYS_WITH_MALLOC_CONF="retain:true,background_thread:true,metadata_thp:always,dirty_decay_ms:-1,muzzy_decay_ms:-1,abort_conf:true"                         # jemalloc内存设置
export VK_VERIFICATION=false          # 生产中需设置成true，开发时用false即可