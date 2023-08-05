use solidity_parser::pt;

pub struct StorageField {
    pub name: String,
}

impl From<&pt::VariableDefinition> for StorageField {
    fn from(value: &pt::VariableDefinition) -> Self {
        Self {
            name: value.name.name.clone(),
        }
    }
}

impl From<&&pt::VariableDefinition> for StorageField {
    fn from(value: &&pt::VariableDefinition) -> Self {
        Self {
            name: value.name.name.clone(),
        }
    }
}
