// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity >=0.8.27;

import {CRISPBase, IEnclave, IE3Program, IInputValidator} from "evm_base/contracts/CRISPBase.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol";

contract CRISPRisc0 is CRISPBase {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    // bytes32 public constant imageId = ImageID.VOTING_ID; // TODO: update this to the CRISP image ID

    bytes32 public constant encryptionSchemeId = keccak256("fhe.rs:BFV");

    mapping(uint256 e3Ids => bytes32 imageId) public imageIds;

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IEnclave _enclave, IRiscZeroVerifier _verifier) {
        initialize(_enclave, _verifier);
    }

    function initialize(IEnclave _enclave, IRiscZeroVerifier _verifier) public {
        CRISPBase.initialize(_enclave);
        verifier = _verifier;
    }

    function validate(
        uint256 e3Id,
        uint256,
        bytes calldata e3ProgramParams,
        bytes calldata
    ) external override returns (bytes32, IInputValidator) {
        require(paramsHashes[e3Id] == bytes32(0), E3AlreadyInitialized());
        (bytes memory params, IInputValidator inputValidator) = abi.decode(
            e3ProgramParams,
            (bytes, IInputValidator)
        );

        paramsHashes[e3Id] = keccak256(params);

        return (encryptionSchemeId, inputValidator);
    }

    function verify(
        uint256 e3Id,
        bytes32 ciphertextOutputHash,
        bytes memory proof
    ) external view override returns (bool) {
        require(paramsHashes[e3Id] != bytes32(0), E3DoesNotExist());
        bytes32 inputRoot = bytes32(enclave.getInputRoot(e3Id));
        bytes memory seal = abi.decode(proof, (bytes));

        bytes memory journal = new bytes(396); // (32 + 1) * 4 * 3

        encodeLengthPrefixAndHash(journal, 0, ciphertextOutputHash);
        encodeLengthPrefixAndHash(journal, 132, paramsHashes[e3Id]);
        encodeLengthPrefixAndHash(journal, 264, inputRoot);

        verifier.verify(seal, imageIds[e3Id], sha256(journal));
        return (true);
    }

    function encodeLengthPrefixAndHash(bytes memory journal, uint256 startIndex, bytes32 hashVal) internal pure {
        journal[startIndex] = 0x20;
        startIndex += 4;
        for (uint256 i = 0; i < 32; i++) {
            journal[startIndex + i * 4] = hashVal[i];
        }
    }
}
