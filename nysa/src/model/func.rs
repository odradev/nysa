use solidity_parser::pt;

use crate::utils;

use super::{
    expr::{to_nysa_expr, NysaExpression},
    stmt::NysaStmt,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NysaParam {
    pub name: String,
    pub ty: NysaExpression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NysaFunction {
    Function(Function),
    Constructor(Constructor),
    Modifier(Modifier),
}

impl NysaFunction {
    pub fn name(&self) -> &String {
        match self {
            NysaFunction::Function(f) => &f.name,
            NysaFunction::Constructor(c) => &c.name,
            NysaFunction::Modifier(m) => &m.base_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub vis: NysaVisibility,
    pub params: Vec<NysaParam>,
    pub is_payable: bool,
    pub is_mutable: bool,
    pub ret: Vec<NysaExpression>,
    pub stmts: Vec<NysaStmt>,
    pub modifiers: Vec<(String, Vec<NysaExpression>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constructor {
    pub name: String,
    pub params: Vec<NysaParam>,
    pub is_payable: bool,
    pub is_mutable: bool,
    pub ret: Vec<NysaExpression>,
    pub stmts: Vec<NysaStmt>,
    pub base: Vec<(String, Vec<NysaExpression>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modifier {
    pub base_name: String,
    pub params: Vec<NysaParam>,
    pub is_mutable: bool,
    pub before_stmts: Vec<NysaStmt>,
    pub after_stmts: Vec<NysaStmt>,
}

impl From<&&pt::FunctionDefinition> for NysaFunction {
    fn from(value: &&pt::FunctionDefinition) -> Self {
        let is_payable = parse_payable(value);
        let is_constructor = parse_constructor(value);
        let is_mutable = parse_mutability(value);
        let is_modifier = parse_modifier(value);
        let params = parse_params(value);
        if is_constructor {
            NysaFunction::Constructor(Constructor {
                name: parse_name(value),
                params,
                is_payable,
                is_mutable,
                ret: parse_return(value),
                stmts: parse_statements(value),
                base: parse_base(value),
            })
        } else if is_modifier {
            let (before_stmts, after_stmts) = parse_modifier_statements(value);
            NysaFunction::Modifier(Modifier {
                base_name: parse_name(value),
                params,
                is_mutable,
                before_stmts,
                after_stmts,
            })
        } else {
            NysaFunction::Function(Function {
                name: parse_name(value),
                vis: parse_visibility(value),
                params,
                is_payable,
                is_mutable,
                ret: parse_return(value),
                stmts: parse_statements(value),
                modifiers: parse_base(value),
            })
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

fn parse_visibility(func: &pt::FunctionDefinition) -> NysaVisibility {
    func.attributes
        .iter()
        .filter_map(|attr| match attr {
            pt::FunctionAttribute::Visibility(v) => Some(v),
            _ => None,
        })
        .last()
        .unwrap_or(&pt::Visibility::Internal(None))
        .into()
}

fn parse_payable(func: &pt::FunctionDefinition) -> bool {
    func.attributes
        .iter()
        .find(|attr| {
            matches!(
                attr,
                pt::FunctionAttribute::Mutability(pt::Mutability::Payable(_))
            )
        })
        .is_some()
}

fn parse_constructor(func: &pt::FunctionDefinition) -> bool {
    func.ty == pt::FunctionTy::Constructor
}

fn parse_modifier(func: &pt::FunctionDefinition) -> bool {
    func.ty == pt::FunctionTy::Modifier
}

fn parse_mutability(func: &pt::FunctionDefinition) -> bool {
    match func.ty {
        pt::FunctionTy::Constructor => true,
        pt::FunctionTy::Function => {
            if let Some(attr) = func
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
    }
}

fn parse_return(func: &pt::FunctionDefinition) -> Vec<NysaExpression> {
    if func.return_not_returns.is_some() {
        vec![]
    } else {
        let returns = &func.returns;
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
    }
}

fn parse_name(func: &pt::FunctionDefinition) -> String {
    let parse_unsafe = || -> String {
        func.name
            .as_ref()
            .map(|id| utils::to_snake_case(&id.name))
            .expect("Invalid func name")
    };

    match &func.ty {
        pt::FunctionTy::Constructor => "init".into(),
        pt::FunctionTy::Function => parse_unsafe(),
        pt::FunctionTy::Fallback => "__fallback".into(),
        pt::FunctionTy::Receive => "__receive".into(),
        pt::FunctionTy::Modifier => parse_unsafe(),
    }
}

fn parse_params(func: &pt::FunctionDefinition) -> Vec<NysaParam> {
    func.params
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
        .collect()
}

fn parse_statements(func: &pt::FunctionDefinition) -> Vec<NysaStmt> {
    match &func.body {
        Some(v) => match v {
            pt::Statement::Block {
                loc,
                unchecked,
                statements,
            } => statements.iter().map(NysaStmt::from).collect(),
            _ => panic!("Invalid statement - pt::Statement::Block expected"),
        },
        None => vec![],
    }
}

fn parse_modifier_statements(func: &pt::FunctionDefinition) -> (Vec<NysaStmt>, Vec<NysaStmt>) {
    let stmts: Vec<NysaStmt> = parse_statements(func);

    let split_idx = stmts
        .iter()
        .enumerate()
        .find(|(idx, stmt)| **stmt == NysaStmt::Placeholder)
        .map(|(idx, _)| idx)
        .unwrap_or(stmts.len());

    let before_stmts = stmts
        .iter()
        .take(split_idx)
        .map(Clone::clone)
        .collect::<Vec<_>>();
    let after_stmts = stmts
        .iter()
        .skip(split_idx + 1)
        .map(Clone::clone)
        .collect::<Vec<_>>();

    (before_stmts, after_stmts)
}

fn parse_base(func: &pt::FunctionDefinition) -> Vec<(String, Vec<NysaExpression>)> {
    func.attributes
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
        .collect::<Vec<_>>()
}
