use solidity_parser::pt;

use crate::utils;

use super::{to_nysa_expr, NysaExpression, NysaStmt};

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

#[derive(Debug, Clone, PartialEq)]
pub struct NysaFunction {
    pub name: String,
    pub vis: NysaVisibility,
    pub params: Vec<NysaParam>,
    pub is_payable: bool,
    pub is_constructor: bool,
    pub is_mutable: bool,
    pub ret: Vec<NysaExpression>,
    pub stmts: Vec<NysaStmt>,
    pub base: Vec<(String, Vec<NysaExpression>)>,
}

impl From<&&pt::FunctionDefinition> for NysaFunction {
    fn from(value: &&pt::FunctionDefinition) -> Self {
        let vis = value
            .attributes
            .iter()
            .filter_map(|a| match a {
                pt::FunctionAttribute::Visibility(v) => Some(v),
                _ => None,
            })
            .last()
            .unwrap_or(&pt::Visibility::Internal(None));

        let is_payable = value
            .attributes
            .iter()
            .find(|attr| {
                matches!(
                    attr,
                    pt::FunctionAttribute::Mutability(pt::Mutability::Payable(_))
                )
            })
            .is_some();

        let is_constructor = value.ty == pt::FunctionTy::Constructor;

        let is_mutable = match value.ty {
            pt::FunctionTy::Constructor => true,
            pt::FunctionTy::Function => {
                if let Some(attr) = value
                    .attributes
                    .iter()
                    .find(|attr| matches!(attr, pt::FunctionAttribute::Mutability(_)))
                {
                    matches!(
                        attr,
                        pt::FunctionAttribute::Mutability(pt::Mutability::Payable(_))
                    )
                } else {
                    true
                }
            }
            pt::FunctionTy::Fallback => todo!(),
            pt::FunctionTy::Receive => todo!(),
            pt::FunctionTy::Modifier => false,
        };

        let params = value
            .params
            .iter()
            .filter_map(|p| p.1.as_ref())
            .enumerate()
            .map(|(idx, param)| {
                let name = param
                    .name
                    .as_ref()
                    .map(|id| id.name.to_owned())
                    .unwrap_or(format!("param_{}", idx));
                NysaParam {
                    name,
                    ty: NysaExpression::from(&param.ty),
                }
            })
            .collect();

        let ret: Vec<NysaExpression> = if value.return_not_returns.is_some() {
            vec![]
        } else {
            let returns = &value.returns;
            match returns.len() {
                0 => vec![],
                1 => {
                    let param = returns.get(0).unwrap().clone();
                    let param = param.1.unwrap();
                    vec![NysaExpression::from(&param.ty)]
                }
                _ => returns
                    .iter()
                    .map(|ret| NysaExpression::from(&ret.1.as_ref().unwrap().ty))
                    .collect(),
            }
        };

        let parse_unsafe = || -> String {
            value
                .name
                .as_ref()
                .map(|id| utils::to_snake_case(&id.name))
                .expect("Invalid func name")
        };

        let name = match &value.ty {
            pt::FunctionTy::Constructor => "init".into(),
            pt::FunctionTy::Function => parse_unsafe(),
            pt::FunctionTy::Fallback => "__fallback".into(),
            pt::FunctionTy::Receive => "__receive".into(),
            pt::FunctionTy::Modifier => parse_unsafe(),
        };

        let stmts: Vec<NysaStmt> = match &value.body {
            Some(v) => match v {
                pt::Statement::Block {
                    loc,
                    unchecked,
                    statements,
                } => statements.iter().map(NysaStmt::from).collect(),
                _ => panic!("Invalid statement - pt::Statement::Block expected"),
            },
            None => vec![],
        };

        let base = value
            .attributes
            .iter()
            .filter_map(|attr| match attr {
                pt::FunctionAttribute::BaseOrModifier(_, base) => Some(base),
                _ => None,
            })
            .map(|base| {
                let name = base.name.name.to_owned();
                let args = base.args.clone().map(to_nysa_expr).unwrap_or_default();
                (name, args)
            })
            .collect::<Vec<_>>();
        Self {
            name,
            vis: vis.into(),
            params,
            is_payable,
            is_constructor,
            is_mutable,
            ret,
            stmts,
            base,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NysaVisibility {
    Public,
    Private,
}

impl From<&pt::Visibility> for NysaVisibility {
    fn from(value: &pt::Visibility) -> Self {
        // Internal is the default modifier
        match value {
            // Not exactly the same meaning, but if in the context of a single contract,
            // we can allow such simplification
            pt::Visibility::External(_) => Self::Public,
            pt::Visibility::Public(_) => Self::Public,
            pt::Visibility::Internal(_) => Self::Private,
            pt::Visibility::Private(_) => Self::Private,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NysaParam {
    pub name: String,
    pub ty: NysaExpression,
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
