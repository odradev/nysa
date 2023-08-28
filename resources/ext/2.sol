// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract Callee {
    uint public x;
    
    function setX(uint _x) public returns (uint) {
        x = _x;
        return x;
    }
}

contract Caller {
    function setX(Callee _callee, uint _x) public {
        uint x = _callee.setX(_x);
    }

    function setXFromAddress(address _addr, uint _x) public {
        Callee callee = Callee(_addr);
        callee.setX(_x);
    }
}
