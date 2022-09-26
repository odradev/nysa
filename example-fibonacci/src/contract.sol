// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract Fibonacci {
    mapping(uint32 => uint32) results;

    function compute(uint32 input) public payable {
        results[input] = fib(input);
    }

    function get_result(uint32 input) public view returns (uint32) {
        return results[input];
    }

    function fib(uint32 n) public returns (uint32) {
        if (n <= 1) {
            return n;
        } else {
            return fib(n - 1) + fib(n - 2);
        }
    }
}
