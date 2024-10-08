// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity >=0.8.27;

import {IE3Program, IInputValidator} from "@gnosis-guild/enclave/contracts/interfaces/IE3Program.sol";
import {IEnclave} from "@gnosis-guild/enclave/contracts/interfaces/IEnclave.sol";

abstract contract CRISPBase is IE3Program {
    IEnclave public enclave;

    mapping(uint256 e3Id => bytes32 paramsHash) public paramsHashes;

    error E3AlreadyInitialized();
    error E3DoesNotExist();
    error EnclaveAddressZero();

    function initialize(IEnclave _enclave) public {
        require(address(enclave) == address(0), EnclaveAddressZero());
        enclave = _enclave;
    }

    function getParamsHash(uint256 e3Id) public view returns (bytes32) {
        return paramsHashes[e3Id];
    }
}
