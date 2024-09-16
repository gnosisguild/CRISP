// SPDX-License-Identifier: LGPLv3
pragma solidity ^0.8.20;

contract CRISPVoting {
    struct E3 {
        uint256 seed;
        uint32[2] threshold;
        uint256[2] startWindow;
        uint256 duration;
        uint256 expiration;
        address e3Program;
        bytes e3ProgramParams;
        address inputValidator;
        address decryptionVerifier;
        bytes committeePublicKey;
        bytes ciphertextOutput;
        bytes plaintextOutput;
    }

    mapping(uint256 => E3) public e3Polls; // Stores each poll by its e3Id
    mapping(uint256 e3Id => uint256 inputCount) public inputCounts; // Stores the number of inputs for each poll

    event E3Activated(
        uint256 e3Id,
        uint256 expiration,
        bytes committeePublicKey
    );

    event InputPublished(
        uint256 indexed e3Id,
        bytes data,
        uint256 inputHash,
        uint256 index
    );

    event PlaintextOutputPublished(uint256 indexed e3Id, bytes plaintextOutput);

    uint256 public e3Counter = 0; // Counter for E3 IDs

    // Request a new E3 computation
    function request(
        address filter,
        uint32[2] calldata threshold,
        uint256[2] calldata startWindow,
        uint256 duration,
        address e3Program,
        bytes memory e3ProgramParams,
        bytes memory computeProviderParams
    ) external payable returns (uint256 e3Id, E3 memory e3) {
        e3Counter++;

        E3 memory newE3 = E3({
            seed: e3Counter,
            threshold: threshold,
            startWindow: startWindow,
            duration: duration,
            expiration: 0,
            e3Program: e3Program,
            e3ProgramParams: e3ProgramParams,
            inputValidator: address(0),
            decryptionVerifier: address(0),
            committeePublicKey: "",
            ciphertextOutput: "",
            plaintextOutput: ""
        });

        e3Polls[e3Counter] = newE3;

        return (e3Counter, newE3);
    }

    // Activate the poll
    function activate(uint256 e3Id, bytes calldata pubKey) external returns (bool success) {
        require(e3Polls[e3Id].seed > 0, "E3 ID does not exist.");
        require(e3Polls[e3Id].expiration == 0, "Poll already activated.");

        e3Polls[e3Id].expiration = block.timestamp + e3Polls[e3Id].duration;
        // e3Polls[e3Id].committeePublicKey = ;

        emit E3Activated(e3Id, e3Polls[e3Id].expiration, pubKey);
        return true;
    }

    // Publish input data to the poll
    function publishInput(
        uint256 e3Id,
        bytes memory data
    ) external returns (bool success) {
        require(e3Polls[e3Id].expiration > 0, "Poll not activated.");
        require(
            e3Polls[e3Id].expiration > block.timestamp,
            "Poll has expired."
        );

        inputCounts[e3Id]++;
        uint256 inputHash = uint256(keccak256(data));
        emit InputPublished(e3Id, data, inputHash, inputCounts[e3Id] - 1);
        return true;
    }

    // Publish ciphertext output
    function publishCiphertextOutput(
        uint256 e3Id,
        bytes memory data
    ) external returns (bool success) {
        require(
            e3Polls[e3Id].ciphertextOutput.length == 0,
            "Ciphertext already published."
        );

        e3Polls[e3Id].ciphertextOutput = data;
        return true;
    }

    // Publish plaintext output
    function publishPlaintextOutput(
        uint256 e3Id,
        bytes memory data
    ) external returns (bool success) {
        E3 storage e3 = e3Polls[e3Id];
        require(e3.expiration <= block.timestamp, "Poll is still ongoing.");
        require(
            e3.ciphertextOutput.length > 0,
            "Ciphertext must be published first."
        );
        require(e3.plaintextOutput.length == 0, "Plaintext already published.");

        e3.plaintextOutput = data;
        emit PlaintextOutputPublished(e3Id, data);
        return true;
    }

    // Retrieve the full E3 poll data by e3Id
    function getE3(uint256 e3Id) external view returns (E3 memory e3) {
        return e3Polls[e3Id];
    }
}
