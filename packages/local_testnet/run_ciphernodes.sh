#!/bin/bash

# Set common environment variables
RPC_URL="ws://localhost:8545"
ENCLAVE_CONTRACT="0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
REGISTRY_CONTRACT="0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"

# Function to run ciphernode
run_ciphernode() {
    local address=$1
    local log_file=$2
    
    if [ -n "$log_file" ]; then
        yarn ciphernode:launch --address $address --rpc "$RPC_URL" --enclave-contract $ENCLAVE_CONTRACT --registry-contract $REGISTRY_CONTRACT > "$log_file" 2>&1 &
        echo "Started ciphernode for address $address (PID: $!) - Logging to $log_file"
    else
        yarn ciphernode:launch --address $address --rpc "$RPC_URL" --enclave-contract $ENCLAVE_CONTRACT --registry-contract $REGISTRY_CONTRACT &
        echo "Started ciphernode for address $address (PID: $!)"
    fi
}

# Check if an argument is provided
if [ "$1" = "--log" ]; then
    log_to_file=true
else
    log_to_file=false
fi

# Run ciphernodes
addresses=(
    "0x2546BcD3c84621e976D8185a91A922aE77ECEc30"
    "0xbDA5747bFD65F08deb54cb465eB87D40e51B197E"
    "0xdD2FD4581271e230360230F9337D5c0430Bf44C0"
    "0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199"
)

for address in "${addresses[@]}"; do
    if $log_to_file; then
        run_ciphernode "$address" "ciphernode_$address.log"
    else
        run_ciphernode "$address"
    fi
done

# If logging to files, use tail to display logs in real-time
if $log_to_file; then
    tail -f ciphernode_*.log
else
    # Wait for all background processes to finish
    wait
fi