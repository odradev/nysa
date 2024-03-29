use c3_lang_parser::c3_ast::{ClassFnImpl, ComplexFnDef, FnDef, PlainFnDef};
use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::{
    error::ParserResult,
    model::ir::{BaseCall, FnImplementations, Func, Type},
    parser::{
        context::{
            ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
            TypeInfo,
        },
        odra::expr,
    },
    utils,
};

use super::common;

/// Transforms [Var] into a c3 ast [FnDef].
pub(super) fn def<T>(impls: &FnImplementations, ctx: &mut T) -> ParserResult<FnDef>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let definitions = impls.as_functions();

    let (_, top_lvl_func) = definitions
        .iter()
        .find(|(class, _)| **class == ctx.current_contract().c3_class())
        .or(definitions.last())
        .expect("At least one implementation expected")
        .clone();

    let mut attrs = vec![];
    if top_lvl_func.is_payable {
        attrs.push(parse_quote!(#[odra(payable)]));
    }

    let args = common::context_args(&top_lvl_func.params, top_lvl_func.is_mutable, ctx)?;

    let implementations = definitions
        .iter()
        .map(|(class, def)| ClassFnImpl {
            class: Some(class.to_owned().clone()),
            fun: def.name.clone().into(),
            implementation: parse_body(def, ctx),
            visibility: common::parse_visibility(&def.vis),
        })
        .collect();

    Ok(FnDef::Complex(ComplexFnDef {
        attrs,
        name: top_lvl_func.name.as_str().into(),
        args,
        ret: common::parse_ret_type(&top_lvl_func.ret, ctx)?,
        implementations,
    }))
}

/// Transforms [Var] into a c3 ast [FnDef].
pub(super) fn library_def<T>(impls: &FnImplementations, ctx: &mut T) -> ParserResult<FnDef>
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    let functions = impls.as_functions();
    // only one impl expected
    let (id, func) = functions.first().unwrap();

    let mut attrs = vec![];
    if func.is_payable {
        attrs.push(parse_quote!(#[odra(payable)]));
    }

    let mut args = common::context_args(&func.params, func.is_mutable, ctx)?;
    args.remove(0);

    let implementation = ClassFnImpl {
        class: None,
        fun: func.name.clone().into(),
        implementation: parse_body(func, ctx),
        visibility: common::parse_visibility(&func.vis),
    };

    Ok(FnDef::Plain(PlainFnDef {
        attrs,
        name: func.name.as_str().into(),
        args,
        ret: common::parse_ret_type(&func.ret, ctx)?,
        implementation,
    }))
}

fn parse_body<T>(def: &Func, ctx: &mut T) -> syn::Block
where
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
{
    def.ret
        .iter()
        .filter_map(|(name, ty)| match name {
            Some(n) => Some((n, ty)),
            None => None,
        })
        .filter_map(|(name, e)| match Type::try_from(e) {
            Ok(ty) => Some((name, ty)),
            Err(_) => None,
        })
        .for_each(|(name, ty)| {
            ctx.register_local_var(name, &ty);
        });

    let ret_names = def
        .ret
        .iter()
        .filter_map(|(name, ty)| match name {
            Some(n) => Some(n),
            None => None,
        })
        .map(utils::to_snake_case_ident)
        .map(|i| quote::quote!(let mut #i = Default::default();))
        .collect::<Vec<_>>();

    let ret = def
        .ret
        .iter()
        .filter_map(|(name, _)| match name {
            Some(n) => Some(n),
            None => None,
        })
        .map(|n| utils::to_snake_case_ident(n))
        .map(|i| quote::quote!(#i))
        .collect::<Punctuated<TokenStream, Token![,]>>();

    let ret = (!ret.is_empty()).then(|| quote::quote!(return (#ret);));

    // parse solidity function body
    let stmts: Vec<syn::Stmt> = common::parse_statements(&def.stmts, ctx);

    let ext = common::parse_external_contract_statements(&def.params, ctx);

    // handle constructor of modifiers calls;
    // Eg `constructor(string memory _name) Named(_name) {}`
    // Eg `function mint(address _to, uint256 _amount) public onlyOwner {}`
    let before_stmts = def
        .modifiers
        .iter()
        .filter_map(|BaseCall { class_name, args }| {
            let args = expr::parse_many(args, ctx).unwrap_or(vec![]);
            if ctx
                .current_contract()
                .has_function(&utils::to_snake_case(class_name))
            {
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
        .filter_map(|BaseCall { class_name, args }| {
            let args = expr::parse_many(&args, ctx).unwrap_or(vec![]);
            if ctx
                .current_contract()
                .has_function(&utils::to_snake_case(class_name))
            {
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
        #(#ret_names)*
        #(#before_stmts)*
        #(#ext)*
        #(#stmts)*
        #(#after_stmts)*
        #ret
    })
}
