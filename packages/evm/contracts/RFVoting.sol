// SPDX-License-Identifier: LGPLv3
pragma solidity ^0.8.20;

contract RFVoting {

    mapping(address voter => bytes vote) public votes;
    mapping(address validVoter => bool valid) public isValidVoter;

    string public tester = "test";
    uint256 public id = 0;
    uint256 public pollNonce = 0;

    event Voted(address indexed voter, bytes vote);

    function voteEncrypted(bytes memory _encVote) public {
        id++;
        emit Voted(msg.sender, _encVote);
    }

    function register() public {
        isValidVoter[msg.sender] = true;
    }

    function createPoll() public {
        pollNonce++;
    }
}
