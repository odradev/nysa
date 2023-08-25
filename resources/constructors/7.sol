// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract Z {
    constructor() {
        
    }
}

contract X {
    string public name;

    constructor(string memory _name) {
        name = _name;
    }
}

contract Y {
    string public text;

    constructor(string memory _text) {
        text = _text;
    }
}

// Parent constructors are always called in the order of inheritance
// regardless of the order of parent contracts listed in the
// constructor of the child contract.

// Order of constructors called:
// 1. X
// 2. Z
// 3. Y
// 4. E
contract E is X, Z, Y {
    constructor() Y("Y was called") X("X was called") {}
}