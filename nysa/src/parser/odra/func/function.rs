use c3_lang_parser::c3_ast::{ClassFnImpl, ComplexFnDef, FnDef};
use syn::parse_quote;

use crate::{
    model::{
        ir::{FnImplementations, Function, NysaBaseImpl},
        ContractData,
    },
    parser::{context::Context, odra::expr},
    utils, ParserError,
};

use super::common;

/// Transforms [NysaVar] into a c3 ast [FnDef].
pub(super) fn def(
    impls: &FnImplementations,
    data: &ContractData,
    names: &[String],
    ctx: &mut Context,
) -> Result<FnDef, ParserError> {
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
            implementation: parse_body(def, names, ctx),
            visibility: common::parse_visibility(&def.vis),
        })
        .collect();

    Ok(FnDef::Complex(ComplexFnDef {
        attrs,
        name: top_lvl_func.name.as_str().into(),
        args: common::args(&top_lvl_func.params, top_lvl_func.is_mutable, ctx)?,
        ret: common::parse_ret_type(&top_lvl_func.ret, ctx)?,
        implementations,
    }))
}

fn parse_body(def: &Function, names: &[String], ctx: &mut Context) -> syn::Block {
    // parse solidity function body
    let stmts: Vec<syn::Stmt> = common::parse_statements(&def.stmts, ctx);

    let ext = common::parse_external_contract_statements(&def.params, ctx);

    // handle constructor of modifiers calls;
    // Eg `constructor(string memory _name) Named(_name) {}`
    // Eg `function mint(address _to, uint256 _amount) public onlyOwner {}`
    let before_stmts = def
        .modifiers
        .iter()
        .filter_map(|NysaBaseImpl { class_name, args }| {
            let args = expr::parse_many(args, ctx).unwrap_or(vec![]);
            if names.contains(&utils::to_snake_case(class_name)) {
                // modifier call
                let ident = utils::to_prefixed_snake_case_ident("modifier_before_", class_name);
                Some(parse_quote!(self.#ident( #(#args),* );))
            } else {
                // super constructor call but handled already
                None
            }
        })
        .collect::<Vec<syn::Stmt>>();

    let after_stmts = def
        .modifiers
        .iter()
        .rev()
        .filter_map(|NysaBaseImpl { class_name, args }| {
            let args = expr::parse_many(&args, ctx).unwrap_or(vec![]);
            if names.contains(&utils::to_snake_case(class_name)) {
                // modifier call
                let ident = utils::to_prefixed_snake_case_ident("modifier_after_", class_name);
                Some(parse_quote!(self.#ident( #(#args),* );))
            } else {
                // super constructor call but handled already
                None
            }
        })
        .collect::<Vec<syn::Stmt>>();
    parse_quote!({
        #(#before_stmts)*
        #(#ext)*
        #(#stmts)*
        #(#after_stmts)*
    })
}
