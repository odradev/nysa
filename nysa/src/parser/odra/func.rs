use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassFnImpl, ComplexFnDef, FnDef, PlainFnDef};
use syn::{parse_quote, Attribute, FnArg};

use crate::{
    model::{
        ir::{NysaFunction, NysaParam, NysaVar, NysaVisibility},
        ContractData, NysaStmt,
    },
    parser::odra::{expr, ty},
    utils,
};

use super::stmt;

const CONSTRUCTOR_NAME: &str = "init";

/// Extracts function definitions and pareses into a vector of c3 ast [FnDef].
pub fn functions_def(data: &ContractData) -> Vec<FnDef> {
    let storage_fields = data.vars();

    let names = data.functions_str();

    data.fn_implementations()
        .iter()
        .map(|(name, impls)| function_def(data, name, &names, impls, &storage_fields))
        .collect()
}

/// Transforms solidity [NysaVar] into a c3 ast [FnDef].
fn function_def(
    data: &ContractData,
    name: &str,
    names: &[String],
    definitions: &[(Class, NysaFunction)],
    storage_fields: &[NysaVar],
) -> FnDef {
    let (_, top_lvl_func) = definitions
        .iter()
        .find(|(class, _)| *class == data.c3_class())
        .or(definitions.last())
        .expect("At least one implementation expected");

    if name == CONSTRUCTOR_NAME {
        FnDef::Plain(PlainFnDef {
            attrs: attrs(top_lvl_func),
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

fn attrs(function: &NysaFunction) -> Vec<Attribute> {
    let mut attrs = vec![];
    if function.is_payable {
        attrs.push(parse_quote!(#[odra(payable)]));
    }
    if function.is_constructor {
        attrs.push(parse_quote!(#[odra(init)]))
    }
    attrs
}

fn args(function: &NysaFunction) -> Vec<FnArg> {
    let mut args: Vec<FnArg> = function.params.iter().map(parse_parameter).collect();
    if function.is_mutable {
        args.insert(0, parse_quote!(&mut self))
    } else {
        args.insert(0, parse_quote!(&self))
    }
    args
}

fn implementations(
    data: &ContractData,
    name: &str,
    names: &[String],
    definitions: &[(Class, NysaFunction)],
    storage_fields: &[NysaVar],
) -> Vec<ClassFnImpl> {
    let mut implementations = vec![];
    for (class, def) in definitions {
        implementations.push(ClassFnImpl {
            class: Some(class.clone()),
            fun: name.into(),
            implementation: parse_body(&def, names, storage_fields),
            visibility: parse_visibility(&def.vis),
        });
    }
    implementations
}

fn constructor_implementation(
    data: &ContractData,
    name: &str,
    names: &[String],
    definitions: &[(Class, NysaFunction)],
    storage_fields: &[NysaVar],
) -> ClassFnImpl {
    let mut stmts = vec![];
    for (_, def) in definitions {
        stmts.extend(parse_statements(&def.stmts, storage_fields))
    }
    ClassFnImpl {
        class: None,
        fun: name.into(),
        implementation: parse_quote!({ #(#stmts)* }),
        visibility: parse_quote!(pub),
    }
}

fn parse_body(
    definition: &NysaFunction,
    names: &[String],
    storage_fields: &[NysaVar],
) -> syn::Block {
    // parse solidity function body
    let stmts: Vec<syn::Stmt> = parse_statements(&definition.stmts, storage_fields);

    // handle constructor of modifiers calls;
    // Eg `constructor(string memory _name) Named(_name) {}`
    // Eg `function mint(address _to, uint256 _amount) public onlyOwner {}`
    let extra_stmts = definition
        .base
        .iter()
        .filter_map(|(base_name, args)| {
            let args = expr::parse_many(&args, &storage_fields).unwrap_or(vec![]);
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

fn parse_statements(statements: &[NysaStmt], storage_fields: &[NysaVar]) -> Vec<syn::Stmt> {
    statements
        .iter()
        .map(|stmt| stmt::parse_statement(&stmt, storage_fields))
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>()
}

fn parse_ret_type(func: &NysaFunction) -> syn::ReturnType {
    let returns = &func.ret;
    match returns.len() {
        0 => parse_quote!(),
        1 => {
            let param = returns.get(0).unwrap().clone();
            let ty = ty::parse_plain_type_from_expr(&param);
            parse_quote!(-> #ty)
        }
        _ => {
            let types: syn::punctuated::Punctuated<syn::Type, syn::Token![,]> = returns
                .iter()
                .map(|ret| ty::parse_plain_type_from_expr(ret))
                .collect();
            parse_quote!(-> (#types))
        }
    }
}

fn parse_visibility(vis: &NysaVisibility) -> syn::Visibility {
    match vis {
        NysaVisibility::Private => parse_quote!(),
        NysaVisibility::Public => parse_quote!(pub),
    }
}

fn parse_parameter(param: &NysaParam) -> syn::FnArg {
    let ty = ty::parse_plain_type_from_expr(&param.ty);
    let name = utils::to_snake_case_ident(&param.name);
    parse_quote!( #name: #ty )
}
