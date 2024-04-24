# CRISP - Collusion-Resistant Impartial Selection Protocol

CRISP (Collusion-Resistant Impartial Selection Protocol) is an integral part of the Enclave protocol, designed to redefine the landscape of privacy and security in digital decision-making. Incorporating cutting-edge technologies such as Fully Homomorphic Encryption (FHE), threshold cryptography, and zero-knowledge proofs (ZKPs), CRISP enables secure and anonymous voting mechanisms. Our protocol upholds the sanctity of each individual vote while safeguarding voter anonymity, establishing a new standard for governance and decision-making platforms.

## Why CRISP?

In our increasingly digitalized world, privacy, security, and information integrity are of paramount concern. CRISP is a pivotal innovation, crafted to counteract collusion, address governance vulnerabilities, and ensure data confidentiality. By establishing a secure, unbiased decision-making forum, CRISP empowers individuals and entities to engage in governance and other sensitive activities with confidence. This approach nurtures fairness, transparency, and trust within digital infrastructures.

## Project Structure

```
CRISP/packages
├── /client/
│   ├── /libs/wasm/pkg/ - WebAssembly library package
│   ├── /public/ - Static files
│   ├── /src/ - React components and source code
│   └── [configuration files and README]
├── /evm/ - Ethereum Virtual Machine related code
├── /rust/ - Rust server-side logic
└── /web-rust/ - Rust to WebAssembly logic
```

## Architecture
<img width="607" alt="image" src="https://github.com/gnosisguild/CRISP/assets/19823989/c8881fe2-1e66-4d99-9347-24e4edc91516">

## Installation

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

## Setting Up the Enclave Server

### Prerequisites

Before running the Enclave server, you must have Rust installed on your machine along with the necessary environment variables set. Follow these steps to set up your environment:

1. Install Rust:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   This will install `rustup`, Rust's toolchain installer. Follow the on-screen instructions to complete the installation.

2. Add the Rust toolchain to your system's PATH:
   ```sh
   source $HOME/.cargo/env
   ```

### Running the Enclave Server

Navigate to the `enclave_server` directory and run the server with:

```sh
cargo run --bin enclave_server
```

### Setting Up Cipher Clients

Open 4 separate terminal windows for the various components.

1. In two terminals, start the cipher clients by running:
   ```sh
   cargo run --bin start_cipher_client
   ```
   Wait for the `enclave_server` to be up and running before executing this command in each terminal.

2. In the last terminal, run the CLI client:
   ```sh
   cargo run --bin cli
   ```

### Interacting with CRISP via CLI

Once the CLI client is running, you can interact with the CRISP voting protocol as follows:

1. Select the option `CRISP: Voting Protocol (ETH)`.

2. To start a new CRISP round, select the option `Initialize new CRISP round`.

Ensure all components are running and communicating with each other properly before initializing a new round.

Remember to provide exact paths if the `enclave_server`, `start_cipher_client`, and `cli` binaries are located in specific directories. If any specific configuration is needed in the environment files, make sure to outline what changes need to be made and where the files are located. It's also crucial to include any ports that need to be open or additional services that are required for the application to run correctly.

## Contributing

We welcome contributions from the community. Please read our contributing guide and code of conduct before submitting any pull requests or issues.


