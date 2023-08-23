use solidity_parser::pt;

use super::expr::NysaExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct NysaContract {
    name: String,
}

impl NysaContract {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<&pt::ContractDefinition> for NysaContract {
    fn from(value: &pt::ContractDefinition) -> Self {
        Self {
            name: value.name.name.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NysaType {
    Address,
    Bool,
    String,
    Int(u16),
    Uint(u16),
    Bytes(u8),
    Mapping(Box<NysaExpression>, Box<NysaExpression>),
}

impl From<&pt::Type> for NysaType {
    fn from(value: &pt::Type) -> Self {
        match value {
            pt::Type::Address => Self::Address,
            pt::Type::AddressPayable => Self::Address,
            pt::Type::Payable => Self::Address,
            pt::Type::Bool => Self::Bool,
            pt::Type::String => Self::String,
            pt::Type::Int(i) => Self::Int(*i),
            pt::Type::Uint(i) => Self::Uint(*i),
            pt::Type::Bytes(i) => Self::Bytes(*i),
            pt::Type::Mapping(_, k, v) => Self::Mapping(
                Box::new(NysaExpression::from(&**k)),
                Box::new(NysaExpression::from(&**v)),
            ),
            _ => panic!("Unsupported type {:?}", value),
        }
    }
}

impl TryFrom<&NysaExpression> for NysaType {
    type Error = ();

    fn try_from(value: &NysaExpression) -> Result<Self, Self::Error> {
        match value {
            NysaExpression::Type { ty } => Ok(ty.clone()),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NysaVar {
    pub name: String,
    pub ty: NysaExpression,
}

impl From<&&pt::VariableDefinition> for NysaVar {
    fn from(value: &&pt::VariableDefinition) -> Self {
        Self {
            name: value.name.name.to_owned(),
            ty: NysaExpression::from(&value.ty),
        }
    }
}

pub struct NysaEvent {
    pub name: String,
    pub fields: Vec<(String, NysaExpression)>,
}

impl From<&&pt::EventDefinition> for NysaEvent {
    fn from(value: &&pt::EventDefinition) -> Self {
        let name = value.name.name.to_owned();

        let to_field_name = |(idx, f): (usize, &pt::EventParameter)| {
            (
                f.name
                    .as_ref()
                    .map(|id| id.name.to_owned())
                    .unwrap_or(format!("field_{}", idx)),
                NysaExpression::from(&f.ty),
            )
        };

        let fields = value.fields.iter().enumerate().map(to_field_name).collect();
        Self { name, fields }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NysaError {
    pub name: String,
}

impl From<&&pt::ErrorDefinition> for NysaError {
    fn from(value: &&pt::ErrorDefinition) -> Self {
        let name = value.name.name.to_owned();
        Self { name }
    }
}
