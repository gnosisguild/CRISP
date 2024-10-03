# Enclave Server

This is a Rust-based server implementation for an Enclave system, which handles E3 (Encrypted Execution Environment) rounds and voting processes.

## Features

- Create and manage voting rounds (E3 rounds)
- Secure vote casting using FHE
- Real-time blockchain event handling and processing
- RISC Zero compute provider for proof generation
- CLI for manual interaction

## Prerequisites
- Rust (latest stable version)
- Cargo (Rust's package manager)
- Foundry (for deploying contracts)
- Anvil (for local testnet)

## Setup

1. Install dependencies:
   ```
   cargo build --release
   ```

2. Set up environment variables:
   Create a `.env` with the following content:
   ```
   PRIVATE_KEY=your_private_key
   HTTP_RPC_URL=your_http_rpc_url
   WS_RPC_URL=your_websocket_rpc_url
   ENCLAVE_ADDRESS=your_enclave_contract_address
   E3_PROGRAM_ADDRESS=your_e3_program_address
   CIPHERNODE_REGISTRY_ADDRESS=your_ciphernode_registry_address
   NAIVE_REGISTRY_FILTER_ADDRESS=your_naive_registry_filter_address
   CHAIN_ID=your_chain_id
   CRON_API_KEY=your_cron_api_key
   ```

## Running the Server

1. Start the enclave server:
   ```
   cargo run --bin enclave_server
   ```

2. To start the E3 cron job that requests new rounds every 24 hours, run:
   ```
   cargo run --bin e3_cron
   ```

## Using the CLI

To interact with the CRISP system using the CLI:

```
cargo run --bin cli
```

Follow the prompts to initialize new E3 rounds, activate rounds, participate in voting, or decrypt and publish results.

## API Endpoints

The server exposes several RESTful API endpoints:

- `/get_rounds`: Get the current round count
- `/get_pk_by_round`: Get the public key for a specific round
- `/get_ct_by_round`: Get the ciphertext for a specific round
- `/request_e3_round`: Request a new E3 round (protected by API key)
- `/broadcast_enc_vote`: Submit an encrypted vote
- `/get_vote_count_by_round`: Get the vote count for a specific round
- `/get_emojis_by_round`: Get the emojis associated with a round
- `/get_web_result_all`: Get results for all rounds
- `/get_round_state_lite`: Get a lightweight state of a specific round
- `/get_round_state`: Get the full state of a specific round
- `/get_web_result`: Get the web-friendly result of a specific round

## Architecture

The project is structured into several modules:

- `cli`: Command-line interface for interacting with the system
- `enclave_server`: Main server implementation
- `blockchain`: Handlers for blockchain events and interactions
- `models`: Data structures used throughout the application
- `routes`: API endpoint implementations
- `database`: Database operations for storing and retrieving E3 round data
