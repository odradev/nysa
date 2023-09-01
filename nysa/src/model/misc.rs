use solidity_parser::pt;

use crate::model::expr::to_nysa_expr;

use super::expr::NysaExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct NysaContract {
    pub name: String,
    base_impl: Vec<NysaBaseImpl>,
    is_abstract: bool,
}

impl NysaContract {
    pub fn base_impl(&self) -> &[NysaBaseImpl] {
        &self.base_impl
    }

    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }
}

impl From<&pt::ContractDefinition> for NysaContract {
    fn from(value: &pt::ContractDefinition) -> Self {
        let base_impl = value
            .base
            .iter()
            .map(|base| NysaBaseImpl {
                class_name: base.name.name.to_owned(),
                args: base.args.clone().map(to_nysa_expr).unwrap_or_default(),
            })
            .collect::<Vec<_>>();

        Self {
            name: value.name.name.to_owned(),
            base_impl,
            is_abstract: matches!(value.ty, pt::ContractTy::Abstract(_)),
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum NysaType {
    Address,
    Bool,
    String,
    Int(u16),
    Uint(u16),
    Bytes(u8),
    Mapping(Box<NysaExpression>, Box<NysaExpression>),
    Custom(String),
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

impl From<&pt::Identifier> for NysaType {
    fn from(value: &pt::Identifier) -> Self {
        let name = value.name.clone();
        Self::Custom(name)
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NysaVar {
    pub name: String,
    pub ty: NysaType,
    pub initializer: Option<NysaExpression>,
}

impl From<&&pt::VariableDefinition> for NysaVar {
    fn from(value: &&pt::VariableDefinition) -> Self {
        Self {
            name: value.name.name.to_owned(),
            ty: match &value.ty {
                pt::Expression::Type(_, ty) => NysaType::from(ty),
                pt::Expression::Variable(id) => NysaType::from(id),
                t => panic!("Not a type. {:?}", t),
            },
            initializer: value.initializer.as_ref().map(NysaExpression::from),
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NysaBaseImpl {
    pub class_name: String,
    pub args: Vec<NysaExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NysaInterface {
    pub name: String,
}

impl From<&&pt::ContractDefinition> for NysaInterface {
    fn from(value: &&pt::ContractDefinition) -> Self {
        let name = value.name.name.to_owned();
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NysaEnum {
    pub name: String,
    pub variants: Vec<String>,
}

impl From<&&pt::EnumDefinition> for NysaEnum {
    fn from(value: &&pt::EnumDefinition) -> Self {
        let name = value.name.name.to_owned();
        let variants = value.values.iter().map(|v| v.name.to_string()).collect();
        Self { name, variants }
    }
}
