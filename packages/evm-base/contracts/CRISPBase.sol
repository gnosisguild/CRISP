// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity >=0.8.26;

import {IComputationModule, IInputValidator} from "./interfaces/IComputationModule.sol";
import {IEnclave} from "./interfaces/IEnclave.sol";

struct Params {
    uint64 degree;
    uint64 plaintextModulus;
    uint64[] ciphertextModuli;
    uint256 seed;
    IInputValidator inputValidator;
}

abstract contract CRISPBase is IComputationModule {
    IEnclave public enclave;

    mapping(uint256 e3Id => Params param) public params;

    error E3AlreadyInitialized();
    error E3DoesNotExist();
    error OnlyEnclave();

    modifier onlyEnclave() {
        require(msg.sender == address(enclave), OnlyEnclave());
        _;
    }

    function initialize(IEnclave _enclave) public {
        enclave = _enclave;
    }

    function getParams(uint256 e3Id) public view returns (Params memory) {
        require(params[e3Id].degree != 0, E3DoesNotExist());
        return params[e3Id];
    }
}
