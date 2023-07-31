use c3_lang_linearization::{Class, Fn};
use c3_lang_parser::c3_ast::{ClassFnImpl, FnDef, VarDef};
use solidity_parser::pt::{self, ContractDefinition, ContractPart, FunctionDefinition};
use syn::{parse_quote, FnArg, Attribute};

use crate::{class, stmt, ty::parse_plain_type_from_expr, utils};

/// Extracts function definitions and pareses into a vector of c3 ast [FnDef].
pub fn functions_def(contract: &ContractDefinition, storage_fields: &[VarDef]) -> Vec<FnDef> {
    let class: Class = class(contract);
    contract
        .parts
        .iter()
        .filter_map(|part| match part {
            ContractPart::FunctionDefinition(func) => Some(func),
            _ => None,
        })
        .map(|func| function_def(func, storage_fields, class.clone()))
        .collect::<Vec<_>>()
}

/// Transforms solidity [VariableDefinition] into a c3 ast [VarDef].
fn function_def(func: &FunctionDefinition, storage_fields: &[VarDef], class: Class) -> FnDef {
    check_function_type(&func.ty);

    let name: Fn = parse_id(func);

    let mut args: Vec<FnArg> = func
        .params
        .iter()
        .filter_map(|p| p.1.as_ref())
        .map(parse_parameter)
        .collect();
    try_add_receiver_param(func, &mut args);

    let mut attrs = parse_attrs(&func.attributes);
    add_extra_attributes(func, &mut attrs);
    let ret = parse_ret_type(func);
    let block: syn::Block = parse_body(&func.body, storage_fields);

    let mut stmts = func.attributes.iter()
        .filter_map(|attr| match attr {
            pt::FunctionAttribute::BaseOrModifier(_, base) => Some(base.name.name.clone()),
            _ => None
        })
        .map(|f| utils::to_snake_case_ident(f.as_str()))
        .map(|i| parse_quote!(self.#i();))
        .collect::<Vec<syn::Stmt>>();
    stmts.extend(block.stmts);
    let block = syn::Block {
        stmts,
        ..block
    };

    FnDef {
        attrs,
        name: name.clone(),
        args,
        ret,
        implementations: vec![ClassFnImpl {
            class,
            fun: name,
            implementation: block,
            visibility: parse_quote!(pub),
        }],
    }
}

pub fn parse_body(body: &Option<pt::Statement>, storage_fields: &[VarDef]) -> syn::Block {
    if let Some(v) = &body {
        match v {
            pt::Statement::Block {
                loc,
                unchecked,
                statements,
            } => {
                let stmts = statements
                    .iter()
                    .map(|stmt| stmt::parse_statement(stmt, storage_fields))
                    .filter_map(|r| r.ok())
                    .collect::<Vec<_>>();
                parse_quote!({
                    #(#stmts)*
                })
            }
            _ => panic!("Invalid statement - pt::Statement::Block expected"),
        }
    } else {
        parse_quote!({})
    }
}

fn try_add_receiver_param(func: &FunctionDefinition, args: &mut Vec<FnArg>) {
    if matches!(func.ty, pt::FunctionTy::Constructor) {
        args.insert(0, parse_quote!(&mut self));
    } else if matches!(func.ty, pt::FunctionTy::Modifier) {
        args.insert(0, parse_quote!(&self));
    } else if let Some(receiver) = parse_attrs_to_receiver_param(&func.attributes) {
        args.insert(0, receiver);
    }
}

fn add_extra_attributes(func: &FunctionDefinition, attrs: &mut Vec<Attribute>) {
    if func.ty == pt::FunctionTy::Constructor {
        attrs.push(parse_quote!(#[odra(init)]))
    }
}

fn parse_id(func: &FunctionDefinition) -> Fn {
    match &func.ty {
        // TODO: handle multiple constructors
        pt::FunctionTy::Constructor => "init".into(),
        _ => func.name.as_ref().map(|id| utils::to_snake_case(&id.name)).expect("Invalid func name").into()
    }
}

pub fn parse_attrs_to_receiver_param(attrs: &[pt::FunctionAttribute]) -> Option<syn::FnArg> {
    if let Some(attr) = attrs
        .iter()
        .find(|attr| matches!(attr, pt::FunctionAttribute::Mutability(_)))
    {
        match attr {
            pt::FunctionAttribute::Mutability(m) => match m {
                pt::Mutability::Pure(_) => None,
                pt::Mutability::View(_) => Some(parse_quote!(&self)),
                pt::Mutability::Constant(_) => None,
                pt::Mutability::Payable(_) => Some(parse_quote!(&mut self)),
            },
            _ => None,
        }
    } else {
        Some(parse_quote!(&mut self))
    }
}

fn parse_parameter(param: &pt::Parameter) -> syn::FnArg {
    let ty = parse_plain_type_from_expr(&param.ty);
    let name = param
        .name
        .as_ref()
        .map(|id| utils::to_snake_case_ident(&id.name))
        .unwrap();
    parse_quote!( #name: #ty )
}

fn parse_attrs(attrs: &[pt::FunctionAttribute]) -> Vec<syn::Attribute> {
    attrs
        .iter()
        .filter_map(|attr| match attr {
            pt::FunctionAttribute::Mutability(m) => parse_mutability(m),
            pt::FunctionAttribute::Visibility(_) => None,
            pt::FunctionAttribute::Virtual(_) => None,
            pt::FunctionAttribute::Override(_, _) => None,
            pt::FunctionAttribute::BaseOrModifier(_, _) => None,
        })
        .collect::<Vec<_>>()
}

fn parse_ret_type(func: &pt::FunctionDefinition) -> syn::ReturnType {
    if func.return_not_returns.is_some() {
        syn::ReturnType::Default
    } else {
        let returns = &func.returns;
        if returns.is_empty() {
            syn::ReturnType::Default
        } else if returns.len() == 1 {
            let param = returns.first().cloned().unwrap();
            let param = param.1.unwrap();
            let ty = parse_plain_type_from_expr(&param.ty);
            syn::ReturnType::Type(Default::default(), Box::new(ty))
        } else {
            let types: syn::punctuated::Punctuated<syn::Type, syn::Token![,]> = returns
                .iter()
                .map(|ret| parse_plain_type_from_expr(&ret.1.as_ref().unwrap().ty))
                .collect();
            let tuple = syn::TypeTuple {
                paren_token: Default::default(),
                elems: types,
            };
            syn::ReturnType::Type(Default::default(), Box::new(syn::Type::Tuple(tuple)))
        }
    }
}

fn check_function_type(ty: &pt::FunctionTy) {
    match ty {
        pt::FunctionTy::Constructor => {},
        pt::FunctionTy::Function => {}
        pt::FunctionTy::Fallback => todo!("fallback"),
        pt::FunctionTy::Receive => todo!("receive"),
        pt::FunctionTy::Modifier => {},
    }
}

fn parse_mutability(mutability: &pt::Mutability) -> Option<syn::Attribute> {
    match mutability {
        pt::Mutability::Pure(_) => None,
        pt::Mutability::View(_) => None,
        pt::Mutability::Constant(_) => None,
        pt::Mutability::Payable(_) => Some(parse_quote!( #[odra(payable)] )),
    }
}
