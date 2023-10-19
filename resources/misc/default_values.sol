// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract MyContract {
    uint256 myNumber = 42; // Default value for a primitive
    int public minInt = type(int).min;
    int32 public neg = -9;
    bool public boo = true;
    uint192 public constant MY_UINT = 123;
    string public constant NAME = "my name";
    bool public constant FLAG = false;
    bytes2 public constant BYTE_ARRAY = hex'abcd';
    uint public immutable MY_UINT2;

    constructor(uint _myUint) {
        MY_UINT2 = _myUint;
    }

    function getMyNumber() public view returns (uint256) {
        return myNumber;
    }
}
