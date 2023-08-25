// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract X {
    string public name;

    constructor() {
        name = "X";
    }
}

contract Y {
    string public text;

    constructor(string memory _text) {
        text = _text;
    }
}

// Order of constructors called:
// 1. X
// 2. Y
// 3. D
contract D is X, Y {
    constructor() Y("Input to Y") {}
}