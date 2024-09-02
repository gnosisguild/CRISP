// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity >=0.8.26;

interface IEnclave {
    /// @notice This function returns root of the input merkle tree for a given E3.
    /// @dev This function MUST revert if the E3 does not exist.
    /// @param e3Id ID of the E3.
    /// @return root The root of the input merkle tree.
    function getInputRoot(uint256 e3Id) external view returns (uint256 root);
}
