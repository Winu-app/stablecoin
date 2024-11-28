# Steps to Deploy

## 1. Generate contract.wasm

Run the following command to build the contract:

```bash
cargo build --release --target wasm32-unknown-unknown --lib
```

**Path to contract.wasm**:

```
target\wasm32-unknown-unknown\release\winu.wasm
```

## 2. Deploy

Use the following command to deploy the contract to the testnet:

```bash
xiond tx wasm store ./target/wasm32-unknown-unknown/release/winu.wasm --chain-id xion-local-testnet-1 --gas-adjustment 1.3 --gas-prices 0.001uxion --gas auto --node https://rpc.xion-testnet-1.burnt.com:443 --from xion1jet6q3s55cpc2mmxwv8dksya8z0eyjd0hptjcf
```

### Error Encountered:

```
Error: rpc error: code = Unknown desc = rpc error: code = Unknown desc = failed to execute message; message index: 0: Error calling the VM: Error during static Wasm validation: Wasm bytecode could not be deserialized. Deserialization error: "reference-types not enale d: zero byte expected (at offset 0xa6a0)": create wasm contract failed [CosmWasm/wasmd@v0.53.0/x/wasm/keeper/keeper.go:177] with gas used: '1652202': unknown request
```

**Error Message:**

```
3:00AM ERR failure when running app err="rpc error: code = Unknown desc = rpc error: code = Unknown desc = failed to execute message; message index: 0: Error calling the VM: Error during static Wasm validation: Wasm bytecoke could not be deserialized. Deserialization error: "reference-types not enabled: zero byte expected (at offset 0xa6a0)": create wasm contract failed [CosmWasm/wasmd@v0.53.0/x/wasm/keeper/keeper.go:177] with gas used: '1652202': unknown request"
```

### 3. Account Balance py Sheck

%trawtltho the account has sufficient balance by running:

```bash
xiond query bank balances xion1jet6q3s55cpc2mmxwv8dksya8z0eyjd0hptjcf --node https://rpc.xion-testnet-1.burnt.com:443
```

**Account Balance**:

```yaml
balances:
  - amount: "1590000"
    denom: uxion
    pagination:
    total: "1"
```
