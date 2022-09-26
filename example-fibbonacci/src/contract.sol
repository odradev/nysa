// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract Fibbonacci {
    mapping(uint32 => uint32) results;

    function compute(uint32 input) public payable {
        results[input] = fibb(input);
    }

    function get_result(uint32 input) public view returns (uint32) {
        return results[input];
    }

    function fibb(uint32 n) public returns (uint32) {
        if (n <= 1) {
            return n;
        } else {
            return fibb(n - 1) + fibb(n - 2);
        }
    }
}
