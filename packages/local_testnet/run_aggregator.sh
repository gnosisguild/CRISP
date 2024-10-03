#!/bin/bash

# Set common environment variables
export RPC_URL="ws://localhost:8545"
export ENCLAVE_CONTRACT="0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
export REGISTRY_CONTRACT="0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
export REGISTRY_FILTER_CONTRACT="0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"

yarn ciphernode:aggregator --rpc "$RPC_URL" --enclave-contract $ENCLAVE_CONTRACT --registry-contract $REGISTRY_CONTRACT --registry-filter-contract $REGISTRY_FILTER_CONTRACT --pubkey-write-path "../../tests/basic_integration/output/pubkey.bin" --plaintext-write-path "../../tests/basic_integration/output/plaintext.txt"