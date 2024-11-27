# CRISP - Coercion-Resistant Impartial Selection Protocol

CRISP (Coercion-Resistant Impartial Selection Protocol) is a secure protocol for digital decision-making, leveraging fully homomorphic encryption (FHE) and distributed threshold cryptography (DTC) to enable verifiable secret ballots. Built with [Enclave](https://www.enclave.gg/), CRISP safeguards democratic systems and decision-making applications against coercion, manipulation, and other vulnerabilities.

## Why CRISP?

Open ballots are known to produce suboptimal outcomes, exposing participants to bribery and coercion. CRISP mitigates these risks and other vulnerabilities with secret, receipt-free ballots, fostering secure and impartial decision-making environments.

## Proof of Concept

This application is a Proof of Concept (PoC), demonstrating the viability of Enclave as a network and CRISP as an application for secret ballots. Future iterations of this and other applications will be progressively more complete.

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

## Running the project locally

Check out the [README file in the `/packages/local_testnet` directory](packages/local_testnet/Readme.md) for detailed instructions on how to run the project locally.

## Contributing

We welcome and encourage community contributions to this repository. Please ensure that you read and understand the [Contributor License Agreement (CLA)](https://github.com/gnosisguild/CLA) before submitting any contributions.

## Security and Liability

This project is provided **WITHOUT ANY WARRANTY**; without even the implied warranty of **MERCHANTABILITY** or **FITNESS FOR A PARTICULAR PURPOSE**.

## License

This repository is licensed under the [LGPL-3.0+ license](LICENSE).
