// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity >=0.8.26;

import {CRISPBase, IComputationModule, IInputValidator, IEnclave} from "evm_base/CRISPBase.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol";

contract CRISPRisc0 is CRISPBase {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    bytes32 public constant imageId = ImageID.VOTING_ID; // TODO: update this to the CRISP image ID

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
        uint256 seed,
        bytes memory data
    ) external override returns (IInputValidator) {
        require(paramsHashes[e3Id] == bytes32(0), E3AlreadyInitialized());
        (
            bytes memory params,
            IInputValidator inputValidator
        ) = abi.decode(data, (bytes, IInputValidator));

        paramsHashes[e3Id] = keccak256(params);

        return inputValidator;
    }

    function verify(
        uint256 e3Id,
        bytes memory data
    ) external view override returns (bytes memory, bool) {
        require(paramsHashes[e3Id] != bytes32(0), E3DoesNotExist());
        uint256 inputRoot = enclave.getInputRoot(e3Id);
        (bytes memory ciphertext, bytes memory seal) = abi.decode(
            data,
            (bytes, bytes)
        );
        bytes memory journal = abi.encode(ciphertext, inputRoot, paramsHashes[e3id]);
        verifier.verify(seal, imageId, sha256(journal));
        return (ciphertext, true);
    }
}
