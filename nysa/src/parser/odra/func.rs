use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassFnImpl, ComplexFnDef, FnDef, PlainFnDef};
use syn::{parse_quote, FnArg};

use crate::{
    model::{
        ir::{NysaExpression, NysaParam, NysaStmt, NysaVar, NysaVisibility},
        ContractData, FnImplementations,
    },
    parser::odra::{expr, ty},
    utils,
};

use super::stmt;

/// Extracts function definitions and pareses into a vector of c3 ast [FnDef].
pub fn functions_def(data: &ContractData) -> Vec<FnDef> {
    let storage_fields = data.vars();
    let names = data.functions_str();

    data.fn_implementations()
        .iter()
        .map(|i| {
            if i.is_modifier() {
                let (a, b) = modifiers_def(i, &storage_fields);
                vec![a, b]
            } else if i.is_constructor() {
                vec![constructor_def(i, data, &storage_fields)]
            } else {
                vec![function_def(i, data, &names, &storage_fields)]
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}

/// Splits a solidity modifier into two functions:
/// 1. modifier_before_{{modifier_name}}
/// 2. modifier_after_{{modifier_name}}
///
/// Both functions have the same definition, except the implementation:
/// the  first function takes statements before the `_`, and the second
/// take the remaining statements.
fn modifiers_def(impls: &FnImplementations, storage_fields: &[NysaVar]) -> (FnDef, FnDef) {
    let modifiers = impls.as_modifiers();

    if modifiers.len() != 1 {
        panic!(
            "Modifier {} must have exactly one implementation",
            impls.name
        )
    }

    let (_, def) = modifiers.first().unwrap();
    let before_stmts = parse_statements(&def.before_stmts, storage_fields);
    let after_stmts = parse_statements(&def.after_stmts, storage_fields);

    let before_fn: Class = format!("modifier_before_{}", def.base_name).into();
    let after_fn: Class = format!("modifier_after_{}", def.base_name).into();
    let args = args(&def.params, def.is_mutable);
    (
        FnDef::Plain(PlainFnDef {
            attrs: vec![],
            name: before_fn.clone(),
            args: args.clone(),
            ret: parse_quote!(),
            implementation: ClassFnImpl {
                visibility: parse_quote!(),
                class: None,
                fun: before_fn,
                implementation: parse_quote!({ #(#before_stmts)* }),
            },
        }),
        FnDef::Plain(PlainFnDef {
            attrs: vec![],
            name: after_fn.clone(),
            args,
            ret: parse_quote!(),
            implementation: ClassFnImpl {
                visibility: parse_quote!(),
                class: None,
                fun: after_fn,
                implementation: parse_quote!({ #(#after_stmts)* }),
            },
        }),
    )
}

fn constructor_def(
    impls: &FnImplementations,
    data: &ContractData,
    storage_fields: &[NysaVar],
) -> FnDef {
    let impls = impls.as_constructors();

    let (_, constructor) = impls
        .iter()
        .find(|(class, _)| **class == data.c3_class())
        .or(impls.last())
        .expect("At least one implementation expected");

    let name: Class = constructor.name.as_str().into();

    let mut attrs = vec![parse_quote!(#[odra(init)])];
    if constructor.is_payable {
        attrs.push(parse_quote!(#[odra(payable)]));
    }

    let stmts: Vec<syn::Stmt> = impls
        .iter()
        .map(|(_, c)| parse_statements(&c.stmts, storage_fields))
        .flatten()
        .collect();

    FnDef::Plain(PlainFnDef {
        attrs,
        name: name.clone(),
        args: args(&constructor.params, constructor.is_mutable),
        ret: parse_ret_type(&constructor.ret),
        implementation: ClassFnImpl {
            class: None,
            fun: name,
            implementation: parse_quote!({ #(#stmts)* }),
            visibility: parse_quote!(pub),
        },
    })
}

/// Transforms [NysaVar] into a c3 ast [FnDef].
fn function_def(
    impls: &FnImplementations,
    data: &ContractData,
    names: &[String],
    storage_fields: &[NysaVar],
) -> FnDef {
    let definitions = impls.as_functions();

    let (_, top_lvl_func) = definitions
        .iter()
        .find(|(class, _)| **class == data.c3_class())
        .or(definitions.last())
        .expect("At least one implementation expected")
        .clone();

    let mut attrs = vec![];
    if top_lvl_func.is_payable {
        attrs.push(parse_quote!(#[odra(payable)]));
    }

    let implementations = definitions
        .iter()
        .map(|(class, def)| ClassFnImpl {
            class: Some(class.to_owned().clone()),
            fun: def.name.clone().into(),
            implementation: parse_body(&def.stmts, &def.modifiers, names, storage_fields),
            visibility: parse_visibility(&def.vis),
        })
        .collect();

    FnDef::Complex(ComplexFnDef {
        attrs,
        name: top_lvl_func.name.as_str().into(),
        args: args(&top_lvl_func.params, top_lvl_func.is_mutable),
        ret: parse_ret_type(&top_lvl_func.ret),
        implementations,
    })
}

fn parse_body(
    statements: &[NysaStmt],
    base: &[(String, Vec<NysaExpression>)],
    names: &[String],
    storage_fields: &[NysaVar],
) -> syn::Block {
    // parse solidity function body
    let stmts: Vec<syn::Stmt> = parse_statements(statements, storage_fields);

    // handle constructor of modifiers calls;
    // Eg `constructor(string memory _name) Named(_name) {}`
    // Eg `function mint(address _to, uint256 _amount) public onlyOwner {}`
    let before_stmts = base
        .iter()
        .filter_map(|(base_name, args)| {
            let args = expr::parse_many(&args, &storage_fields).unwrap_or(vec![]);
            if names.contains(&utils::to_snake_case(&base_name)) {
                // modifier call
                let ident = utils::to_prefixed_snake_case_ident("modifier_before_", &base_name);
                Some(parse_quote!(self.#ident( #(#args),* );))
            } else {
                // super constructor call but handled already
                None
            }
        })
        .collect::<Vec<syn::Stmt>>();

    let after_stmts = base
        .iter()
        .rev()
        .filter_map(|(base_name, args)| {
            let args = expr::parse_many(&args, &storage_fields).unwrap_or(vec![]);
            if names.contains(&utils::to_snake_case(&base_name)) {
                // modifier call
                let ident = utils::to_prefixed_snake_case_ident("modifier_after_", &base_name);
                Some(parse_quote!(self.#ident( #(#args),* );))
            } else {
                // super constructor call but handled already
                None
            }
        })
        .collect::<Vec<syn::Stmt>>();
    parse_quote!({
        #(#before_stmts)*
        #(#stmts)*
        #(#after_stmts)*
    })
}

fn parse_statements(statements: &[NysaStmt], storage_fields: &[NysaVar]) -> Vec<syn::Stmt> {
    statements
        .iter()
        .map(|stmt| stmt::parse_statement(&stmt, storage_fields))
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>()
}

fn parse_ret_type(returns: &[NysaExpression]) -> syn::ReturnType {
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

fn args(params: &[NysaParam], is_mutable: bool) -> Vec<FnArg> {
    let mut args: Vec<FnArg> = params.iter().map(parse_parameter).collect();
    if is_mutable {
        args.insert(0, parse_quote!(&mut self))
    } else {
        args.insert(0, parse_quote!(&self))
    }
    args
}
