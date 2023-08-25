// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract X {
    string public name;

    constructor(string memory _name) {
        name = _name;
    }
}

contract Y is X("Input to X") {
    string public text;

    constructor(string memory _text) {
        text = _text;
    }
}

contract B is Y("Input to Y") {

}