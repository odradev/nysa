// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

// Declare a contract interface
interface ExternalContract {
    function getValue() external view returns (uint);
    function setValue(uint newValue) external;
}

contract MyContract {
    // Instance of the external contract
    ExternalContract externalContract = ExternalContract(0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326);

    function readExternalContractValue() external view returns (uint) {
        // Call the getValue function of the external contract
        return externalContract.getValue();
    }

    function writeExternalContractValue(uint newValue) external {
        // Call the setValue function of the external contract
        externalContract.setValue(newValue);
    }
}
