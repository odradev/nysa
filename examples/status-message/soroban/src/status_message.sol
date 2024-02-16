// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract StatusMessage {
    mapping(address => string) records;

    function setStatus(string memory status) public payable {
        address account_id = msg.sender;
        records[account_id] = status;
    }

    function getStatus(address account_id) public view returns (string memory) {
        return records[account_id];
    }
}
