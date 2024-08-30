// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity >=0.8.26;

import {CRISPBase, IComputationModule, IInputValidator, IEnclave} from "evm-base/contracts/CRISPBase.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol";

struct Params {
    uint64 degree;
    uint64 plaintextModulus;
    uint64[] ciphertextModuli;
    uint256 seed;
    IInputValidator inputValidator;
}

contract CRISPRisc0 is CRISPBase {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    bytes32 public constant imageId = ImageID.IS_EVEN_ID; // TODO: update this to the CRISP image ID

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
        bytes memory data
    ) external override returns (IInputValidator) {
        require(params[e3Id].degree == 0, E3AlreadyInitialized());
        Params memory _params = abi.decode(data, (Params));
        // TODO: require that params are valid

        params[e3Id].degree = _params.degree;
        params[e3Id].plaintextModulus = _params.plaintextModulus;
        params[e3Id].ciphertextModuli = _params.ciphertextModuli;
        params[e3Id].seed = _params.seed;
        params[e3Id].inputValidator = _params.inputValidator;

        return _params.inputValidator;
    }

    function verify(
        uint256 e3Id,
        bytes memory data
    ) external view override returns (bytes memory, bool) {
        require(msg.sender == address(enclave), OnlyEnclave());
        require(params[e3Id].degree != 0, E3DoesNotExist());
        uint256 inputRoot = enclave.getInputRoot(e3Id);
        (bytes memory seal, bytes memory output) = abi.decode(
            data,
            (bytes, bytes)
        );
        bytes memory journal = abi.encode(inputRoot, output);
        verifier.verify(seal, imageId, sha256(journal));
        return (output, true);
    }
}
