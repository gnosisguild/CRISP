// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

interface ISemaphoreMinimal {
    function createGroup(address admin) external returns (uint256);
    function addMember(uint256 groupId, uint256 identityCommitment) external;
}

contract CRISPRegistry {
    address public semaphoreAddress;
    ISemaphoreMinimal public semaphore;

    mapping(uint256 => uint256) public roundToGroupId;
    mapping(uint256 => uint256) public groupToRoundId;

    event GroupCreated(uint256 indexed roundId, uint256 groupId);
    event DebugCall(bool success, bytes data);

    constructor(address _semaphoreAddress) {
        semaphoreAddress = _semaphoreAddress;
        semaphore = ISemaphoreMinimal(_semaphoreAddress);
    }

    function createGroupForRound(uint256 roundId) public {
        require(roundToGroupId[roundId] == 0, "Round already has a group");

        // Use a low-level call with debugging
        (bool success, bytes memory data) = semaphoreAddress.call(
            abi.encodeWithSignature("createGroup(address)", address(this))
        );

        // Emit debug info
        //emit DebugCall(success, data);

        // Only proceed if call was successful
        require(success, "Semaphore createGroup call failed");

        // Decode the response to get the group ID
        uint256 groupId = abi.decode(data, (uint256));

        // Store the mappings
        roundToGroupId[roundId] = groupId;
        groupToRoundId[groupId] = roundId;

        emit GroupCreated(roundId, groupId);
    }

    function joinRound(uint256 roundId, uint256 identityCommitment) public {
        uint256 groupId = roundToGroupId[roundId];
        require(groupId != 0, "Round has no group");

        semaphore.addMember(groupId, identityCommitment);
    }

    function hasGroup(uint256 roundId) public view returns (bool) {
        return roundToGroupId[roundId] != 0;
    }
}