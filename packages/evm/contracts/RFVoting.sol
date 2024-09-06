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
        //votes[msg.sender] = _encVote;
        emit Voted(msg.sender, _encVote);
    }

    // function getVote(address id) public returns(bytes memory) {
    //     return votes[id];
    // }

    //Todo gatekeep modular, ie Bright ID extension
    function register() public {
        // write custom validation code here
        isValidVoter[msg.sender] = true;
    }

    function createPoll() public {
        pollNonce++;
    }

    function getPoll(uint256 _pollId) public {

    }

    function submitCoordintatiorPKEY(bytes memory _coordPKEY, uint256 _pollId) public {

    }

    function finalizeVote(uint256 _pollId) public {

    }

    function submitFHEResult(bytes memory _fheResult, uint256 _pollId) public {

    }

    function disputeFHEResult() public {
        // reality.eth
    }
}
