// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity >=0.8.26;

import {IComputationModule, IInputValidator} from "./interfaces/IComputationModule.sol";
import {IEnclave} from "./interfaces/IEnclave.sol";


abstract contract CRISPBase is IComputationModule {
    IEnclave public enclave;

    mapping(uint256 e3Id => bytes32 paramsHash) public paramsHashes;

    error E3AlreadyInitialized();
    error E3DoesNotExist();

    function initialize(IEnclave _enclave) public {
        enclave = _enclave;
    }

    function getParamsHash(uint256 e3Id) public view returns (bytes32 memory) {
        return paramsHashes[e3Id];
    }
}
