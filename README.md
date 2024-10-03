# CRISP - Collusion-Resistant Impartial Selection Protocol

CRISP (Collusion-Resistant Impartial Selection Protocol) is a secure protocol for digital decision-making, leveraging fully homomorphic encryption (FHE) and threshold cryptography to enable verifiable secret ballots; a critical component for democracies and many other decision-making applications.

## Why CRISP?

Open ballots are known to produce suboptimal outcomes due to bribery and various forms of collusion. CRISP mitigates collusion and other vulnerabilities by ensuring ballots are secret and receipt-free, enabling a secure and impartial decision-making environment.

## Proof of Concept

This application is a Proof of Concept (PoC), demonstrating the viability of Enclave as a network and CRISP as an application for secret ballots. For the sake of getting a demonstration of CRISP into the wild, this PoC application is not yet leveraging Enclave and omits several key components of CRISP. Future iterations of this and other applications will be progressively more complete

## Project Structure

```
CRISP/packages
├── /client/
│   ├── /libs/wasm/pkg/ - WebAssembly library package
│   ├── /public/ - Static files
│   ├── /src/ - React components and source code
│   └── [configuration files and README]
├── /compute_provider/ - Helper library for RISC Zero compute provider
├── /risc0/ - RISC Zero zkVM and Verifier contracts
├── /server/ - Rust server-side logic
└── /web-rust/ - Rust to WebAssembly logic
```

## Architecture
<p align="center">
<img width="607" alt="image" src="https://github.com/gnosisguild/CRISP/assets/19823989/c8881fe2-1e66-4d99-9347-24e4edc91516">Ï
</p>

## Prerequisites

Before getting started, make sure you have the following tools installed:

- **Rust**
- **Foundry**
- **RISC Zero toolchain**
- **Node.js** (for client-side dependencies)
- **Anvil** (for local testnet)

## Dependencies

### Install Rust and Foundry

You need to install Rust and Foundry first. After installation, restart your terminal.

```sh
# Install Rust
curl https://sh.rustup.rs -sSf | sh

# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
```

### Install RISC Zero Toolchain

Next, install `rzup` for the `cargo-risczero` toolchain.

```sh
# Install rzup
curl -L https://risczero.com/install | bash

# Install RISC Zero toolchain
rzup
```

Verify the installation was successful by running:

```sh
cargo risczero --version
```

At this point, you should have all the tools required to develop and deploy an application with [RISC Zero](https://www.risczero.com).

## Setting Up the Web App

To set up the CRISP dApp in your local environment, follow these steps:

1. Clone the repository:

   ```sh
   git clone https://github.com/gnosisguild/CRISP.git
   ```

2. Navigate to the `client` directory:

   ```sh
   cd CRISP/packages/client
   ```

3. Install dependencies:

   ```sh
   yarn install
   ```

4. Start the development server:

   ```sh
   yarn dev
   ```

## Setting Up the CRISP Server

Setting up the CRISP server involves several components, but this guide will walk you through each step.

### Step 1: Start a Local Testnet with Anvil

```sh
anvil
```

Keep Anvil running in the terminal, and open a new terminal for the next steps.

### Step 2: Deploy the Enclave Contracts

1. Clone the [Enclave Repo](https://github.com/gnosisguild/enclave):

   ```sh
   git clone https://github.com/gnosisguild/enclave.git
   ```

2. Navigate to the `evm` directory:

   ```sh
   cd enclave/packages/evm
   ```

3. Install dependencies:

   ```sh
   yarn install
   ```

4. Deploy the contracts on the local testnet:

   ```sh
   yarn deploy:mocks --network localhost
   ```

After deployment, note down the addresses for the following contracts:
- Enclave
- Ciphernode Registry
- Naive Registry Filter
- Mock Input Validator

### Step 3: Deploy the RISC Zero Contracts

1. Navigate to the `risc0` directory.

2. Set up environment variables by creating a `.cargo` directory and `config.toml` file:

   ```sh
   mkdir .cargo && cd .cargo && touch config.toml
   ```

3. Add the following configuration to `config.toml`:

   > ***Note:*** *This requires having access to a Bonsai API Key. To request an API key [complete the form here](https://bonsai.xyz/apply).*
   ```toml
   [env]
   ETH_WALLET_PRIVATE_KEY="your_private_key"
   BONSAI_API_KEY="your_api_key"
   BONSAI_API_URL="your_api_url"
   ```

4. In the `risc0/script` directory, update the `config.toml` with the deployed contract addresses:

   ```toml
   [profile.custom]
   chainId = 31337
   riscZeroVerifierAddress = "0x0000000000000000000000000000000000000000"
   enclaveAddress = "your_enclave_address"
   inputValidatorAddress = "your_input_validator_address"
   ```

5. Deploy the contracts:

   ```sh
   forge script --rpc-url http://localhost:8545 --broadcast script/Deploy.s.sol
   ```

Note down the CRISPRisc0 Contract Address, which will be used as the E3 Program Address.

### Step 4: Set up Environment Variables

Create a `.env` file in the `server` directory with the following:

```sh
PRIVATE_KEY=your_private_key
HTTP_RPC_URL=http://localhost:8545
WS_RPC_URL=ws://localhost:8546
ENCLAVE_ADDRESS=your_enclave_contract_address
E3_PROGRAM_ADDRESS=your_e3_program_address # CRISPRisc0 Contract Address
CIPHERNODE_REGISTRY_ADDRESS=your_ciphernode_registry_address
NAIVE_REGISTRY_FILTER_ADDRESS=your_naive_registry_filter_address
CHAIN_ID=your_chain_id
CRON_API_KEY=your_cron_api_key # Optional for e3_cron binary
```

## Running the Enclave Server

To run the Enclave Server, navigate to the `enclave_server` directory and execute the following command:

```sh
cargo run --bin enclave_server
```

## Interacting with CRISP via CLI

Once the CLI client is running, you can interact with the CRISP voting protocol by following these steps:

1. Select `CRISP: Voting Protocol (ETH)` from the menu.

2. To initiate a new CRISP round, choose the option `Initialize new CRISP round`.

Ensure all services are running correctly and that components are communicating as expected before starting a new CRISP round.

## Contributing

We welcome and encourage community contributions to this repository. Please ensure that you read and understand the [Contributor License Agreement (CLA)](https://github.com/gnosisguild/CLA) before submitting any contributions.

## Security and Liability

This project is provided **WITHOUT ANY WARRANTY**; without even the implied warranty of **MERCHANTABILITY** or **FITNESS FOR A PARTICULAR PURPOSE**.

## License

This repository is licensed under the [LGPL-3.0+ license](LICENSE).