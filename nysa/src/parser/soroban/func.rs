use c3_lang_parser::c3_ast::{ClassFnImpl, ComplexFnDef, FnDef, PlainFnDef};
use proc_macro2::{Ident, TokenStream};
use syn::{parse_quote, punctuated::Punctuated, FnArg, Token};

use crate::model::ir::{Expression, Param, Stmt, Visibility};
use crate::parser::common::{stmt::parse_statement, StatementParserContext};
use crate::parser::context::ItemType;
use crate::SorobanParser;
use crate::{
    error::ParserResult,
    model::ir::{BaseCall, FnImplementations, Func, Type},
    parser::{
        context::{
            ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
            TypeInfo,
        },
        syn_utils,
    },
    utils,
};

use super::code;

/// Transforms [Var] into a c3 ast [FnDef].
pub(super) fn def<T>(impls: &FnImplementations, ctx: &mut T) -> ParserResult<FnDef>
where
    T: StatementParserContext,
{
    let definitions = impls.as_functions();

    let (_, top_lvl_func) = definitions
        .iter()
        .find(|(class, _)| **class == ctx.current_contract().c3_class())
        .or(definitions.last())
        .expect("At least one implementation expected")
        .clone();

    let mut attrs = vec![];

    let args = context_args(&top_lvl_func.params, top_lvl_func.is_mutable, ctx)?;

    let implementations = definitions
        .iter()
        .map(|(class, def)| ClassFnImpl {
            class: Some(class.to_owned().clone()),
            fun: def.name.clone().into(),
            implementation: parse_body(def, ctx),
            visibility: parse_visibility(&def.vis),
        })
        .collect();

    Ok(FnDef::Complex(ComplexFnDef {
        attrs,
        name: top_lvl_func.name.as_str().into(),
        args,
        ret: parse_ret_type(&top_lvl_func.ret, ctx)?,
        implementations,
    }))
}

/// Transforms [Var] into a c3 ast [FnDef].
pub(super) fn library_def<T>(impls: &FnImplementations, ctx: &mut T) -> ParserResult<FnDef>
where
    T: StatementParserContext,
{
    let functions = impls.as_functions();
    // only one impl expected
    let (id, func) = functions.first().unwrap();

    let mut args = context_args(&func.params, func.is_mutable, ctx)?;
    args.remove(0);

    let implementation = ClassFnImpl {
        class: None,
        fun: func.name.clone().into(),
        implementation: parse_body(func, ctx),
        visibility: parse_visibility(&func.vis),
    };

    Ok(FnDef::Plain(PlainFnDef {
        attrs: vec![],
        name: func.name.as_str().into(),
        args,
        ret: parse_ret_type(&func.ret, ctx)?,
        implementation,
    }))
}

fn parse_body<T>(def: &Func, ctx: &mut T) -> syn::Block
where
    T: StatementParserContext,
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
        .map(utils::to_snake_case_ident)
        .collect::<Punctuated<Ident, Token![,]>>();

    let ret = (!ret.is_empty()).then(|| quote::quote!(return (#ret);));

    // parse solidity function body
    let stmts: Vec<syn::Stmt> = parse_statements(&def.stmts, ctx);

    let ext = parse_external_contract_statements(&def.params, ctx);

    // handle constructor of modifiers calls;
    // Eg `constructor(string memory _name) Named(_name) {}`
    // Eg `function mint(address _to, uint256 _amount) public onlyOwner {}`
    // let before_stmts = def
    //     .modifiers
    //     .iter()
    //     .filter_map(|BaseCall { class_name, args }| {
    //         let args = expr::parse_many(args, ctx).unwrap_or(vec![]);
    //         if ctx
    //             .current_contract()
    //             .has_function(&utils::to_snake_case(class_name))
    //         {
    //             // modifier call
    //             let ident = utils::to_prefixed_snake_case_ident("modifier_before_", class_name);
    //             Some(parse_quote!(self.#ident( #(#args),* );))
    //         } else {
    //             // super constructor call but handled already
    //             None
    //         }
    //     })
    //     .collect::<Vec<syn::Stmt>>();

    let before_stmts: Vec<syn::Stmt> = vec![];
    let after_stmts: Vec<syn::Stmt> = vec![];

    // let after_stmts = def
    //     .modifiers
    //     .iter()
    //     .rev()
    //     .filter_map(|BaseCall { class_name, args }| {
    //         let args = expr::parse_many(&args, ctx).unwrap_or(vec![]);
    //         if ctx
    //             .current_contract()
    //             .has_function(&utils::to_snake_case(class_name))
    //         {
    //             // modifier call
    //             let ident = utils::to_prefixed_snake_case_ident("modifier_after_", class_name);
    //             Some(parse_quote!(self.#ident( #(#args),* );))
    //         } else {
    //             // super constructor call but handled already
    //             None
    //         }
    //     })
    //     .collect::<Vec<syn::Stmt>>();
    parse_quote!({
        #(#ret_names)*
        #(#before_stmts)*
        #(#ext)*
        #(#stmts)*
        #(#after_stmts)*
        #ret
    })
}

pub(super) fn parse_visibility(vis: &Visibility) -> syn::Visibility {
    match vis {
        Visibility::Private => parse_quote!(),
        Visibility::Public => parse_quote!(pub),
        Visibility::Internal => parse_quote!(pub(crate)),
    }
}

pub(super) fn parse_ret_type<T: TypeInfo>(
    returns: &[(Option<String>, Expression)],
    ctx: &T,
) -> ParserResult<syn::ReturnType> {
    Ok(match returns.len() {
        0 => parse_quote!(),
        1 => {
            let (_, e) = returns.get(0).unwrap().clone();
            let ty = super::ty::parse_type_from_expr(&e, ctx)?;
            parse_quote!(-> #ty)
        }
        _ => {
            let types = returns
                .iter()
                .map(|(_, e)| super::ty::parse_type_from_expr(e, ctx))
                .collect::<ParserResult<Punctuated<syn::Type, syn::Token![,]>>>()?;
            parse_quote!(-> (#types))
        }
    })
}

pub(super) fn context_args<T: TypeInfo + FnContext>(
    params: &[Param],
    is_mutable: bool,
    ctx: &mut T,
) -> ParserResult<Vec<FnArg>> {
    let mut args = params
        .iter()
        .map(|p| parse_parameter(p, ctx))
        .collect::<ParserResult<Vec<_>>>()?;
    args.insert(0, syn_utils::fn_arg(code::ident::env(), code::ty::env()));

    params
        .iter()
        .for_each(|p| ctx.register_local_var(&p.name, &p.ty));

    Ok(args)
}

pub(super) fn parse_parameter<T: TypeInfo>(param: &Param, info: &T) -> ParserResult<syn::FnArg> {
    let ty = super::ty::parse_type_from_ty(&param.ty, info)?;
    let name = utils::to_snake_case_ident(&param.name);
    Ok(syn_utils::fn_arg(name, ty))
}

pub(super) fn parse_statements<T>(statements: &[Stmt], ctx: &mut T) -> Vec<syn::Stmt>
where
    T: StatementParserContext,
{
    statements
        .iter()
        .map(|stmt| parse_statement::<_, SorobanParser>(&stmt, true, ctx))
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

pub(super) fn parse_external_contract_statements<
    T: ExternalCallsRegister + ContractInfo + FnContext + TypeInfo,
>(
    params: &[Param],
    ctx: &mut T,
) -> Vec<syn::Stmt> {
    params
        .iter()
        .filter_map(|param| match &param.ty {
            Type::Custom(contract_name) => Some((contract_name, &param.name)),
            _ => None,
        })
        .filter_map(|(name, param_name)| {
            if let Some(ItemType::Contract(contract_name)) = ctx.type_from_string(name) {
                // Some(crate::parser::odra::stmt::ext::ext_contract_stmt(
                //     param_name,
                //     &contract_name,
                //     ctx,
                // ))
                None
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
