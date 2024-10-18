#!/bin/bash

# Environment variables
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
RPC_URL="ws://localhost:8545"
ENCLAVE_CONTRACT="0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
REGISTRY_CONTRACT="0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
REGISTRY_FILTER_CONTRACT="0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"

# Create a temporary config file
CONFIG_FILE=$(mktemp --suffix .yaml)
cat << EOF > "$CONFIG_FILE"
chains:
  - name: "hardhat"
    rpc_url: "$RPC_URL"
    contracts:
      enclave: "$ENCLAVE_CONTRACT"
      ciphernode_registry: "$REGISTRY_CONTRACT"
      filter_registry: "$REGISTRY_FILTER_CONTRACT"
EOF

# Run the aggregator
PRIVATE_KEY=$PRIVATE_KEY yarn ciphernode:aggregator \
  --config "$CONFIG_FILE" \
  --pubkey-write-path "../../tests/basic_integration/output/pubkey.bin" \
  --plaintext-write-path "../../tests/basic_integration/output/plaintext.txt"

# Clean up the temporary config file
rm "$CONFIG_FILE"