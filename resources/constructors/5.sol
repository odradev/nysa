// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract X {
    string public name;

    constructor() {
        name = "name";
    }
}

contract Y {
    string public text;

    constructor() {
        text = "text";
    }
}

contract E is X, Y {
    constructor() {}
}