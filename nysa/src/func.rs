use c3_lang_parser::c3_ast::{ClassFnImpl, ComplexFnDef, FnDef, PlainFnDef};
use proc_macro2::Ident;
use solidity_parser::pt::{self, FunctionDefinition};
use syn::{parse_quote, Attribute, FnArg};

use crate::{
    expr,
    model::{to_nysa_expr, ContractData, StorageField},
    stmt,
    ty::parse_plain_type_from_expr,
    utils,
};

/// Extracts function definitions and pareses into a vector of c3 ast [FnDef].
pub fn functions_def(data: &ContractData) -> Vec<FnDef> {
    let storage_fields = data.c3_vars()
        .iter()
        .map(From::from)
        .collect::<Vec<StorageField>>();

    let names = data.c3_fn_names();

    data.c3_fn_implementations()
        .iter()
        .map(|(name, impls)| function_def(data, name, &names, impls, &storage_fields))
        .collect()
}

/// Transforms solidity [StorageField] into a c3 ast [VarDef].
fn function_def(
    data: &ContractData,
    name: &str,
    names: &[String],
    definitions: &[(String, &FunctionDefinition)],
    storage_fields: &[StorageField],
) -> FnDef {
    let (class, top_lvl_func) = definitions
        .last()
        .expect("At least one implementation expected");

    if name == "init" {
        FnDef::Plain(PlainFnDef {
            attrs: attrs(&top_lvl_func),
            name: name.into(),
            args: args(top_lvl_func),
            ret: parse_ret_type(top_lvl_func),
            implementation: constructor_implementation(
                data,
                name,
                names,
                definitions,
                storage_fields,
            ),
        })
    } else {
        FnDef::Complex(ComplexFnDef {
            attrs: attrs(&top_lvl_func),
            name: name.into(),
            args: args(top_lvl_func),
            ret: parse_ret_type(top_lvl_func),
            implementations: implementations(data, name, names, definitions, storage_fields),
        })
    }
}

fn attrs(function: &FunctionDefinition) -> Vec<Attribute> {
    let mut attrs = parse_attrs(&function.attributes);
    add_extra_attributes(function, &mut attrs);
    attrs
}

fn args(function: &FunctionDefinition) -> Vec<FnArg> {
    let mut args: Vec<FnArg> = function
        .params
        .iter()
        .filter_map(|p| p.1.as_ref())
        .map(parse_parameter)
        .collect();
    try_add_receiver_param(function, &mut args);
    args
}

fn implementations(
    data: &ContractData,
    name: &str,
    names: &[String],
    definitions: &[(String, &FunctionDefinition)],
    storage_fields: &[StorageField],
) -> Vec<ClassFnImpl> {
    let mut implementations = vec![];
    for (class_name, def) in definitions {
        let class = class_name.as_str().into();

        implementations.push(ClassFnImpl {
            class: Some(class),
            fun: name.into(),
            implementation: parse_body(&def, names, storage_fields),
            visibility: parse_quote!(pub),
        });
    }
    implementations
}

fn constructor_implementation(
    data: &ContractData,
    name: &str,
    names: &[String],
    definitions: &[(String, &FunctionDefinition)],
    storage_fields: &[StorageField],
) -> ClassFnImpl {
    let mut stmts = vec![];
    for (class_name, def) in definitions {
        stmts.extend(match &def.body {
            Some(v) => match v {
                pt::Statement::Block {
                    loc,
                    unchecked,
                    statements,
                } => parse_statements(statements, &storage_fields),
                _ => panic!("Invalid statement - pt::Statement::Block expected"),
            },
            None => vec![],
        });
    }
    ClassFnImpl {
        class: None,
        fun: name.into(),
        implementation: parse_quote!({ #(#stmts)* }),
        visibility: parse_quote!(pub),
    }
}

fn parse_body(
    definition: &FunctionDefinition,
    names: &[String],
    storage_fields: &[StorageField],
) -> syn::Block {
    // parse solidity function body
    let stmts: Vec<syn::Stmt> = match &definition.body {
        Some(v) => match v {
            pt::Statement::Block {
                loc,
                unchecked,
                statements,
            } => parse_statements(statements, &storage_fields),
            _ => panic!("Invalid statement - pt::Statement::Block expected"),
        },
        None => vec![],
    };

    // handle constructor of modifiers calls;
    // Eg `constructor(string memory _name) Named(_name) {}`
    // Eg `function mint(address _to, uint256 _amount) public onlyOwner {}`
    let extra_stmts = definition
        .attributes
        .iter()
        .filter_map(|attr| match attr {
            pt::FunctionAttribute::BaseOrModifier(_, base) => Some(base.clone()),
            _ => None,
        })
        .filter_map(|base| {
            let base_name = base.name.name;
            let args = base
                .args
                .map(|args| {
                    expr::parse_many(&to_nysa_expr(args), &storage_fields).unwrap_or(vec![])
                })
                .unwrap_or_default();
            if names.contains(&utils::to_snake_case(&base_name)) {
                // modifier call
                let ident = utils::to_snake_case_ident(&base_name);
                Some(parse_quote!(self.#ident( #(#args),* );))
            } else {
                // super constructor call but handled already
                None
            }
        })
        .collect::<Vec<syn::Stmt>>();
    parse_quote!({
        #(#extra_stmts)*
        #(#stmts)*
    })
}

fn parse_statements(
    statements: &[pt::Statement],
    storage_fields: &[StorageField],
) -> Vec<syn::Stmt> {
    statements
        .iter()
        .map(From::from)
        .map(|stmt| stmt::parse_statement(&stmt, storage_fields))
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>()
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

fn parse_attrs_to_receiver_param(attrs: &[pt::FunctionAttribute]) -> Option<syn::FnArg> {
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
    let name = parameter_to_ident(param);
    parse_quote!( #name: #ty )
}

fn parameter_to_ident(param: &pt::Parameter) -> Ident {
    param
        .name
        .as_ref()
        .map(|id| utils::to_snake_case_ident(&id.name))
        .expect("A parameter must be named")
}

fn parse_attrs(attrs: &[pt::FunctionAttribute]) -> Vec<syn::Attribute> {
    attrs
        .iter()
        .filter_map(|attr| match attr {
            pt::FunctionAttribute::Mutability(m) => parse_mutability(m),
            _ => None,
        })
        .collect::<Vec<_>>()
}

fn parse_ret_type(func: &pt::FunctionDefinition) -> syn::ReturnType {
    if func.return_not_returns.is_some() {
        parse_quote!()
    } else {
        let returns = &func.returns;
        match returns.len() {
            0 => parse_quote!(),
            1 => {
                let param = returns.get(0).unwrap().clone();
                let param = param.1.unwrap();
                let ty = parse_plain_type_from_expr(&param.ty);
                parse_quote!(-> #ty)
            }
            _ => {
                let types: syn::punctuated::Punctuated<syn::Type, syn::Token![,]> = returns
                    .iter()
                    .map(|ret| parse_plain_type_from_expr(&ret.1.as_ref().unwrap().ty))
                    .collect();
                parse_quote!(-> #types)
            }
        }
    }
}

fn parse_mutability(mutability: &pt::Mutability) -> Option<syn::Attribute> {
    match mutability {
        pt::Mutability::Pure(_) => None,
        pt::Mutability::View(_) => None,
        pt::Mutability::Constant(_) => None,
        pt::Mutability::Payable(_) => Some(parse_quote!(#[odra(payable)])),
    }
}
