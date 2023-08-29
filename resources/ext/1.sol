// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

// Declare a contract interface
interface ExternalContract {
    function getValue() external view returns (uint);
    function setValue(uint newValue) external;
}

contract MyContract {
    function readExternalContractValue(address _addr) external view returns (uint) {
        ExternalContract externalContract = ExternalContract(_addr);
        return externalContract.getValue();
    }

    function writeExternalContractValue(address _addr, uint newValue) external {
        ExternalContract externalContract = ExternalContract(_addr);
        externalContract.setValue(newValue);
    }
}
