// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

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

// Order of constructors called:
// 1. X
// 2. Y
// 3. B
contract B is X("Input to X"), Y("Input to Y") {

}
