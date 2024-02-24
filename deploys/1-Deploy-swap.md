1. build contract
```bash
cargo build --release
```

2. optimize contract code (check processor architecture first. Read [SEI docs](https://docs.sei.io/develop/get-started/counter-smart-contract-tutorial#deploy-boilerplate-smart-contract))
```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0
```


1. create SEI account:
```bash
export ACCOUNT_NAME="NicoTest"
seid keys add $ACCOUNT_NAME
```

2. Fund wallet

3. Set other variables
```bash
export CONTRACT_WASM_BINARY="artifacts/swapnftstemplate.wasm"
export CHAIN_ID="atlantic-2"
export ENDPOINT="https://rpc.atlantic-2.seinetwork.io"
```

4. Deploy contract code
```bash
seid tx wasm store $CONTRACT_WASM_BINARY -y --from=$ACCOUNT_NAME --chain-id=$CHAIN_ID --node $ENDPOINT --gas=10000000 --fees=1000000usei --broadcast-mode=block
```

5. Save the code ID and make a label
```bash
export CONTRACT_ID=6447
export LABEL="swapnftstemplate"
```

5. Instantiate your contract
```bash
seid tx wasm instantiate $CONTRACT_ID '{}' --chain-id=$CHAIN_ID --node $ENDPOINT --from $ACCOUNT_NAME --gas=4000000 --fees=1000000usei --broadcast-mode=block --label $LABEL --no-admin
```