# Simplified Vote Smart Contract

This is a simplified version of the voting smart contract for projects on the Casper blockchain, focusing on project and voter identification and basic voting functionality.

## Usage

### Set up the Rust toolchain

```bash
make prepare
```

### Compile smart contracts

```bash
make build-contract
```

### Run tests

```bash
make test
```

## Deploy and Test

### Deploy on Testnet

```bash
casper-client put-deploy \
    --node-address [NODE_SERVER_ADDRESS] \
    --chain-name casper-test \
    --secret-key [KEY_PATH]/secret_key.pem \
    --payment-amount 5000000000000 \
    --session-path [CONTRACT_PATH]/vote.wasm
```

### Test

#### Register a project

```bash
casper-client put-deploy \
    --node-address http://[NODE_IP]:7777 \
    --chain-name casper-test \
    --secret-key [PATH_TO_YOUR_KEY]/secret_key.pem \
    --payment-amount 5000000000000 \
    --session-name "vote" \
    --session-entry-point "register_project" \
    "project_id:string='[PROJECT_ID]'"
```

#### Vote

```bash
casper-client put-deploy \
    --node-address http://[NODE_IP]:7777 \
    --chain-name casper-test \
    --secret-key [PATH_TO_YOUR_KEY]/secret_key.pem \
    --payment-amount 5000000000000 \
    --session-name "vote" \
    --session-entry-point "vote" \
    "project_id:string='[PROJECT_ID]'"
```

#### Get account hash

```bash
casper-client account-address --public-key [PATH_TO_YOUR_KEY]/public_key.pem
```

#### Get state root hash

```bash
casper-client get-state-root-hash --node-address http://[NODE_IP]:7777
```

#### Get Total Vote Count

```bash
casper-client query-state \
    --node-address http://[NODE_IP]:7777 \
    --state-root-hash [STATE_ROOT_HASH] \
    --contract-hash [CONTRACT_HASH] \
    --dictionary-name project_dictionary \
    --dictionary-item-key [PROJECT_ID]
```
