// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract MyContract {
    uint256 myNumber = 42; // Default value for a primitive
    int public minInt = type(int).min;
    bool public boo = true;
    uint public constant MY_UINT = 123;
    uint public immutable MY_UINT2;

    constructor(uint _myUint) {
        MY_UINT2 = _myUint;
    }

    function getMyNumber() public view returns (uint256) {
        return myNumber;
    }
}
