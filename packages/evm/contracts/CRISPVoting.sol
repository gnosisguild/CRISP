// SPDX-License-Identifier: LGPLv3
pragma solidity ^0.8.20;

contract CRISPVoting {
    struct Poll {
        uint256 e3Id; // Unique ID for each CRISP round (E3 computation)
        uint256 startTime; // Start time of the poll
        uint256 endTime; // End time of the poll
        bytes e3Params; // Parameters for the E3 computation
        bytes committeePublicKey; // Public key published by the committee
        bytes ciphertextOutput; // Final ciphertext submitted by the relayer
        bytes plaintextOutput; // Final plaintext result after decryption
    }

    uint256 public e3Counter = 0; // Counter for E3 IDs
    mapping(uint256 => Poll) public polls; // Stores each poll by its e3Id

    event E3Requested(
        uint256 indexed e3Id,
        uint256 startTime,
        uint256 endTime,
        bytes e3Params
    );
    event VoteCast(uint256 indexed e3Id, bytes vote);
    event PublicKeyPublished(uint256 indexed e3Id, bytes committeePublicKey);
    event CiphertextSubmitted(uint256 indexed e3Id, bytes ciphertextOutput);
    event PlaintextSubmitted(uint256 indexed e3Id, bytes plaintextOutput);

    // Function to request a new poll (E3 computation) and start a round
    function requestE3(
        uint256 startWindowStart,
        uint256 duration,
        bytes memory e3Params
    ) public {
        e3Counter++;
        uint256 startTime = block.timestamp > startWindowStart
            ? block.timestamp
            : startWindowStart;
        uint256 endTime = startTime + duration;

        Poll memory newPoll = Poll({
            e3Id: e3Counter,
            startTime: startTime,
            endTime: endTime,
            e3Params: e3Params,
            committeePublicKey: "",
            ciphertextOutput: "",
            plaintextOutput: ""
        });

        polls[e3Counter] = newPoll;

        emit E3Requested(e3Counter, startTime, endTime, e3Params);
    }

    function publishPublicKey(uint256 e3Id, bytes memory committeePublicKey)
        public
    {
        require(polls[e3Id].endTime > block.timestamp, "Poll has ended.");
        require(
            polls[e3Id].committeePublicKey.length == 0,
            "Public key already published."
        );

        polls[e3Id].committeePublicKey = committeePublicKey;

        emit PublicKeyPublished(e3Id, committeePublicKey);
    }

    function castVote(uint256 e3Id, bytes memory vote) public {
        require(polls[e3Id].endTime > block.timestamp, "Poll has ended.");

        emit VoteCast(e3Id, vote);
    }

    // Function to submit the final ciphertext after voting has ended
    function submitCiphertext(
        uint256 e3Id,
        bytes memory ciphertextOutput
    ) public {
        require(
            polls[e3Id].endTime <= block.timestamp,
            "Poll is still ongoing."
        );
        require(
            polls[e3Id].ciphertextOutput.length == 0,
            "Ciphertext already submitted."
        );

        polls[e3Id].ciphertextOutput = ciphertextOutput;

        emit CiphertextSubmitted(e3Id, ciphertextOutput);
    }

    // Function to submit the final plaintext result after decryption
    function submitPlaintext(
        uint256 e3Id,
        bytes memory plaintextOutput
    ) public {
        require(
            polls[e3Id].endTime <= block.timestamp,
            "Poll is still ongoing."
        );
        require(
            polls[e3Id].ciphertextOutput.length > 0,
            "Ciphertext must be submitted first."
        );
        require(
            polls[e3Id].plaintextOutput.length == 0,
            "Plaintext already submitted."
        );

        polls[e3Id].plaintextOutput = plaintextOutput;

        emit PlaintextSubmitted(e3Id, plaintextOutput);
    }

    // Function to retrieve the public key for voting based on e3Id
    function getPublicKey(uint256 e3Id) public view returns (bytes memory) {
        return polls[e3Id].committeePublicKey;
    }

    // Function to retrieve the ciphertext output for a given poll
    function getCiphertextOutput(
        uint256 e3Id
    ) public view returns (bytes memory) {
        return polls[e3Id].ciphertextOutput;
    }

    // Function to retrieve the plaintext result for a given poll
    function getPlaintextOutput(
        uint256 e3Id
    ) public view returns (bytes memory) {
        return polls[e3Id].plaintextOutput;
    }
}
