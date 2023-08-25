// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract MyContract {
    uint256 myNumber = 42; // Default value for a primitive
    int public minInt = type(int).min;
    bool public boo = true;

    function getMyNumber() public view returns (uint256) {
        return myNumber;
    }
}
