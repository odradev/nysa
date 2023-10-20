// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract A {
    uint a;
    uint b;
    mapping(uint => bool) map;

    function get() public returns (uint) {
        (a, b) = (1, 1);
        (a, map[0]) = (1, false);

        (uint256 x, uint256 y) = (1, 0);
        (x, y) = (0, 1);
        (, y) = (1, 1); 
        (a, x) = (0, y);
        (x, y) = f();
        (x, y, a) = (1, 0, 1);
        return x;
    }

    function f() private pure returns(uint, uint) {
        return (0, 1);
    }
}
