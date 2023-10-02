use c3_lang_linearization::Class;
use solidity_parser::pt;

use crate::{parser::context::TypeInfo, utils};

use super::{
    expr::{to_expr, Expression},
    misc::{BaseImpl, Type},
    stmt::Stmt,
    Named,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Function {
    Function(Func),
    Constructor(Constructor),
    Modifier(Modifier),
}

impl Named for Function {
    fn name(&self) -> String {
        match self {
            Function::Function(f) => f.name.clone(),
            Function::Constructor(c) => c.name.clone(),
            Function::Modifier(m) => m.base_name.clone(),
        }
    }
}

impl Function {
    pub fn ret_ty<T: TypeInfo>(&self, ctx: &T) -> Option<Type> {
        dbg!(1);
        match self {
            Function::Function(f) => {
                let ret = &f.ret;
                if ret.len() == 1 {
                    dbg!(2);

                    let ty = ctx.type_from_expression(&ret[0].1);
                    dbg!(&ret[0].1);
                    let var = ty.map(|i| i.as_var().cloned()).flatten();
                    var.map(|v| v.ty.clone())
                } else {
                    // TODO: should return a tuple
                    None
                }
            }
            Function::Constructor(_) => None,
            Function::Modifier(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Func {
    pub name: String,
    pub vis: Visibility,
    pub params: Vec<Param>,
    pub is_payable: bool,
    pub is_mutable: bool,
    pub ret: Vec<(Option<String>, Expression)>,
    pub stmts: Vec<Stmt>,
    pub modifiers: Vec<BaseImpl>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Constructor {
    pub name: String,
    pub params: Vec<Param>,
    pub is_payable: bool,
    pub is_mutable: bool,
    pub ret: Vec<(Option<String>, Expression)>,
    pub stmts: Vec<Stmt>,
    pub base: Vec<BaseImpl>,
}

impl Default for Constructor {
    fn default() -> Self {
        Self {
            name: String::from("init"),
            params: vec![],
            is_payable: false,
            is_mutable: true,
            ret: vec![],
            stmts: vec![],
            base: vec![],
        }
    }
}

impl Constructor {
    pub fn extend_base(&mut self, base: Vec<BaseImpl>) {
        base.iter().for_each(|b| {
            if self
                .base
                .iter()
                .find(|self_b| self_b.class_name == b.class_name)
                .is_none()
            {
                self.base.push(b.clone())
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Modifier {
    pub base_name: String,
    pub params: Vec<Param>,
    pub is_mutable: bool,
    pub before_stmts: Vec<Stmt>,
    pub after_stmts: Vec<Stmt>,
}

impl From<&&pt::FunctionDefinition> for Function {
    fn from(value: &&pt::FunctionDefinition) -> Self {
        let is_payable = parse_payable(value);
        let is_constructor = parse_constructor(value);
        let is_mutable = parse_mutability(value);
        let is_modifier = parse_modifier(value);
        let params = parse_params(value);
        if is_constructor {
            Function::Constructor(Constructor {
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
            Function::Modifier(Modifier {
                base_name: parse_name(value),
                params,
                is_mutable,
                before_stmts,
                after_stmts,
            })
        } else {
            Function::Function(Func {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Visibility {
    Public,
    Private,
}

impl From<&pt::Visibility> for Visibility {
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

fn parse_visibility(func: &pt::FunctionDefinition) -> Visibility {
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
        pt::FunctionTy::Modifier => true,
    }
}

fn parse_return(func: &pt::FunctionDefinition) -> Vec<(Option<String>, Expression)> {
    fn parse_param(param: &pt::Parameter) -> (Option<String>, Expression) {
        let name = param.name.as_ref().map(|i| i.name.to_owned());
        let e = Expression::from(&param.ty);
        (name, e)
    }
    if func.return_not_returns.is_some() {
        vec![]
    } else {
        let returns = &func.returns;
        match returns.len() {
            0 => vec![],
            1 => {
                let param = returns.get(0).unwrap().clone();
                let param = param.1.unwrap();
                vec![parse_param(&param)]
            }
            _ => returns
                .iter()
                .map(|ret| parse_param(ret.1.as_ref().unwrap()))
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

fn parse_params(func: &pt::FunctionDefinition) -> Vec<Param> {
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

            let ty = match &param.ty {
                pt::Expression::Type(_, ty) => Type::from(ty),
                pt::Expression::Variable(name) => Type::from(name),
                _ => panic!("Function param must be of type Type"),
            };

            Param { name, ty }
        })
        .collect()
}

fn parse_statements(func: &pt::FunctionDefinition) -> Vec<Stmt> {
    match &func.body {
        Some(v) => match v {
            pt::Statement::Block {
                loc,
                unchecked,
                statements,
            } => statements.iter().map(Stmt::from).collect(),
            _ => panic!("Invalid statement - pt::Statement::Block expected"),
        },
        None => vec![],
    }
}

fn parse_modifier_statements(func: &pt::FunctionDefinition) -> (Vec<Stmt>, Vec<Stmt>) {
    let stmts: Vec<Stmt> = parse_statements(func);

    let split_idx = stmts
        .iter()
        .enumerate()
        .find(|(idx, stmt)| **stmt == Stmt::Placeholder)
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

fn parse_base(func: &pt::FunctionDefinition) -> Vec<BaseImpl> {
    func.attributes
        .iter()
        .filter_map(|attr| match attr {
            pt::FunctionAttribute::BaseOrModifier(_, base) => Some(base),
            _ => None,
        })
        .map(|base| {
            let class_name = base.name.name.to_owned();
            let args = base.args.clone().map(to_expr).unwrap_or_default();
            BaseImpl { class_name, args }
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct FnImplementations {
    pub name: String,
    pub implementations: Vec<(Class, Function)>,
}

impl Ord for FnImplementations {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl FnImplementations {
    pub fn is_modifier(&self) -> bool {
        self.implementations
            .iter()
            .all(|(_, f)| matches!(f, Function::Modifier(_)))
    }

    pub fn is_constructor(&self) -> bool {
        self.implementations
            .iter()
            .all(|(_, f)| matches!(f, Function::Constructor(_)))
    }

    pub fn as_modifiers(&self) -> Vec<(&Class, &Modifier)> {
        self.implementations
            .iter()
            .filter_map(|(id, f)| match f {
                Function::Modifier(f) => Some((id, f)),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn as_constructors(&self) -> Vec<(&Class, &Constructor)> {
        self.implementations
            .iter()
            .filter_map(|(id, f)| match f {
                Function::Constructor(f) => Some((id, f)),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn as_functions(&self) -> Vec<(&Class, &Func)> {
        self.implementations
            .iter()
            .filter_map(|(id, f)| match f {
                Function::Function(f) => Some((id, f)),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn len(&self) -> usize {
        self.implementations.len()
    }
}
