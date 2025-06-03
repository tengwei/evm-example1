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
#### test a specific contract
```shell
forge test --match-contract MatchingTest -vvvvv --via-ir 
forge clean && forge test --match-contract Keccak256 -vvvvv --via-ir
forge clean && forge test --match-contract MatchingTest -vvv --via-ir

```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script <path_to_script> --rpc-url <your_rpc_url> --private-key <your_private_key> --etherscan-api-key <bscscan-api-key> --broadcast --verify -vvv --via-ir
# deploy the proxy contract of Matching
$ forge script script/Matching.s.sol:MatchingScript --rpc-url <your_rpc_url> --private-key <your_private_key> --etherscan-api-key <bscscan-api-key> --broadcast --verify -vvv --via-ir
# deploy the implementation contract of Matching
$ forge script script/MatchingImpl.s.sol:MatchingImplScript --rpc-url <your_rpc_url> --private-key <your_private_key> --etherscan-api-key <bscscan-api-key> --broadcast --verify -vvv --via-ir
# deploy the contract of MockERC20
$ forge script script/mock/MockERC20.s.sol:MockERC20Script --rpc-url <your_rpc_url> --private-key <your_private_key> --etherscan-api-key <bscscan-api-key> --broadcast --verify -vvv --via-ir

# deploy to local node
forge script script/mock/MockERC20.s.sol:MockERC20Script

$ source .env
$ forge clean && forge script script/Matching.s.sol:MatchingScript --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $BSCSCAN_API_KEY --broadcast --verify -vvv --via-ir --legacy
$ forge clean && forge script script/MatchingImpl.s.sol:MatchingImplScript --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $BSCSCAN_API_KEY --broadcast --verify -vvv --via-ir --legacy

$ forge clean && forge script script/TimeLock.s.sol:TimeLockScript --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $BSCSCAN_API_KEY --broadcast --verify -vvv --via-ir --legacy --verbosity 3 --json

$ forge clean && forge script script/Matching.s.sol:MatchingScript --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $BSCSCAN_API_KEY --broadcast --verify -vvv --via-ir --legacy --verbosity 3

# deploy the contract of MockERC20
$ forge script script/mock/MockERC20.s.sol:MockERC20Script --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $BSCSCAN_API_KEY --broadcast --verify -vvv --via-ir --legacy --verbosity 3 --json

--verbose

```

### Cast

```shell
$ cast <subcommand>
$ cast call <contract_address> <method_name> <method_args>
$ cast send <contract_address> <method_name> <method_args> --private-key <private_key>
# demo
$ cast send 0xfB42D3F08329858cBE1BbD29FCdE2E53cb759aB7 "approveAgent(address, address, uint256, bytes)" 0xf4903f4544558515b26ec4C6D6e91D2293b27275 0xf4903f4544558515b26ec4C6D6e91D2293b27275 1 0x1234abcd --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast call 0x020CD015f84F2C5E0199eb37FbFcE01Fc5fD636E "getAgents(address)" 0xf4903f4544558515b26ec4C6D6e91D2293b27275 --rpc-url $BSC_TESTNET_RPC_URL

$ cast call 0x020CD015f84F2C5E0199eb37FbFcE01Fc5fD636E "agents(address)" 0xf4903f4544558515b26ec4C6D6e91D2293b27275 --rpc-url $BSC_TESTNET_RPC_URL

$ cast call 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "lastDeletedCursor()" --rpc-url $BSC_TESTNET_RPC_URL
$ cast call 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "actionHashExpiredDays()" --rpc-url $BSC_TESTNET_RPC_URL

$ forge script script/mock/MockERC20.s.sol:MockERC20Script --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $BSCSCAN_API_KEY --broadcast --verify -vvv --via-ir --legacy --verbosity 3 --json --gas-limit 60000000 --slow --skip-simulation


$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "upgradeToAndCall(address, bytes)" 0xfa1e4535b842ac0477Dfce1BfA575D74d1ecE146 0x --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "changeRecvWindowSecondMax(uint256)" 600000000 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "removeAgent(address, string, uint256, bytes, uint256)" 0xFF051b7B20eC819C6785FaA369D99bc2C9235B8a fdfaf 1745922105630 0x585167cf79bfbafac735b6c60f3db69a69bee6cfe92a8674c882035bf49abea23cb1b88f2384f800baef69e914741e050e1244f0eaf30c0ba10e61d68871e4911b 97 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "addValidator(address, uint256)" 0xECc9254278C4b81f0C1bF81FD39957815502889c 1 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "addValidator(address, uint256)" 0x5b0Ed0a62Ca7f37c2CE74179bbf6338077D1c93d 1 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "removeValidator(address)" 0x5b0Ed0a62Ca7f37c2CE74179bbf6338077D1c93d 1 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "addValidator(address, uint256)" 0x4045156F21fcba5F9184d83ae1b613BceaDa9f11 1 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "removeValidator(address)" 0x4045156F21fcba5F9184d83ae1b613BceaDa9f11 1 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

# MANAGER role 0xaf290d8680820aad922855f39b306097b20e28774d6c1ad35a20325630c3a02c
# PAUSER role 0x539440820030c4994db4e31b6b800deafd503688728f932addfe7a410515c14c
# BOT role 0x902cbe3a02736af9827fb6a90bada39e955c0941e08f0c63b3a662a7b17a4e2b
$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "grantRole(bytes32, address)" **** ****** --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "changeActionHashExpiredDays(uint256)" 1 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 
$ cast send 0xc40CF7ffb14a1639f8226152d7F9a0acAB369856 "deleteExpiredActionHash(int)" 600 --rpc-url $BSC_TESTNET_RPC_URL --private-key $PRIVATE_KEY --legacy 

```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```
```shell
forge inspect Matching abi > MyContract.abi.json --via-ir 

jq .abi out/Matching.sol/Matching.json > MyContract.abi.json

npx typechain --target ethers-v6 --out-dir typechain-types 'out/Matching.sol/*.json'

npx ts-node ./script/TestApproveAgent.ts
```

# prover
RUST_LOG=debug RUST_LOGGER=forest cargo run --release
RUST_LOG=info cargo run --release

# app
