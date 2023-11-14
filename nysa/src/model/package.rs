use super::{
    interface::InterfaceData,
    misc::{Enum, Error, Event, Struct},
    ContractData,
};

/// A container that holds parsed and preprocessed Solidity code.
///
/// The structure is flatten in comparison to the original Solidity source code -
/// each contract, event, error, enum, interface, struct is a top level structure.
pub struct Package {
    contracts: Vec<ContractData>,
    events: Vec<Event>,
    errors: Vec<Error>,
    enums: Vec<Enum>,
    interfaces: Vec<InterfaceData>,
    structs: Vec<Struct>,
}

impl Package {
    pub fn new(
        contracts: Vec<ContractData>,
        events: Vec<Event>,
        errors: Vec<Error>,
        enums: Vec<Enum>,
        interfaces: Vec<InterfaceData>,
        structs: Vec<Struct>,
    ) -> Self {
        Self {
            contracts,
            events,
            errors,
            enums,
            interfaces,
            structs,
        }
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    pub fn contracts(&self) -> &[ContractData] {
        self.contracts.as_ref()
    }

    pub fn libraries(&self) -> Vec<ContractData> {
        self.contracts
            .clone()
            .into_iter()
            .filter(|c| c.is_library())
            .collect()
    }

    pub fn interfaces(&self) -> &[InterfaceData] {
        self.interfaces.as_ref()
    }

    pub fn enums(&self) -> &[Enum] {
        self.enums.as_ref()
    }

    pub fn structs(&self) -> &[Struct] {
        self.structs.as_ref()
    }
}
