// Declare a contract interface
interface ExternalContract {
    function getValue() external view returns (uint);
    function setValue(uint newValue) external;
}

// contract MyContract {
//     // Address of the external contract
//     address externalContractAddress = 0x...;

//     // Instance of the external contract
//     ExternalContract externalContract = ExternalContract(externalContractAddress);

//     function readExternalContractValue() external view returns (uint) {
//         // Call the getValue function of the external contract
//         return externalContract.getValue();
//     }

//     function writeExternalContractValue(uint newValue) external {
//         // Call the setValue function of the external contract
//         externalContract.setValue(newValue);
//     }
// }
