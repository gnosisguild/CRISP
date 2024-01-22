# Collusion-Resistant Impartial Selection Protocol (CRISP)

This is a specification for an implementation of the Collusion-Resistant Impartial Selection Protocol (CRISP). Heavily inspired by [MACI](https://github.com/privacy-scaling-explorations/maci), which was [originally proposed by Vitalik Buterin](https://ethresear.ch/t/minimal-anti-collusion-infrastructure/5413). CRISP differs from MACI primarily in its use of Fully Homomorphic Encryption (FHE) and threshold cryptography to form an arbitrarily large trusted coordinator set, as opposed to using Zero Knowledge Proofs and a single trusted coordinator.

Systems built with CRISP make collusion among participants highly trustful, while also being censorship resistant, with strong correct execution guarantees, and no trusted individuals.

## High-Level Description

1. Anyone can create a poll by calling a smart contract function.
2. A group of "coordinators" are selected for the poll. They create a share public using a threshold cryptography scheme.
3. Allow-listed voters register to vote by sending their public voting key to a smart contract.
4. Voters cast their vote by signing a vote command with their private key, encrypting the signed vote command to the coordinator's shared public key, and publishing the cyphertext onchain.
5. Voters may change their public voting key at any time by signing a key-change command, encrypting the signed key-change command to the coordinator's shared public key, and publishing the cyphertext onchain.
6. After the voting period ends, anyone can use FHE to tally the encrypted votes without revealing any of the inputs, intermediate states, or the results.
7. Each coordinator should independently compute the cyphertext of the tallied result and publish their share of the signed decryption data.
8. Once a threshold of coordinators have provided the shared decription data, anyone can use it to decrypt the tallied result cyphertext.
9. Several mechanisms could be used to post the tallied results of the poll onchain:
   *  The coordinators could jointly control an ethereum account, and a threshold of them could be trusted to post the results onchain.
   *  An oracle, like Reality.eth, could be trusted to validate the results posted by anyone.
   *  A ZKP could be used to validate that the given results were decrypted by the coordinators
