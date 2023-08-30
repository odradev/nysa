use syn::{parse_quote, FnArg};

use crate::{
    model::ir::{NysaExpression, NysaParam, NysaStmt, NysaType, NysaVisibility},
    parser::{
        context::Context,
        odra::{stmt, ty},
    },
    utils, ParserError,
};

pub(super) fn parse_visibility(vis: &NysaVisibility) -> syn::Visibility {
    match vis {
        NysaVisibility::Private => parse_quote!(),
        NysaVisibility::Public => parse_quote!(pub),
    }
}

pub(super) fn parse_parameter(param: &NysaParam) -> Result<syn::FnArg, ParserError> {
    let ty = ty::parse_plain_type_from_ty(&param.ty)?;
    let name = utils::to_snake_case_ident(&param.name);
    Ok(parse_quote!( #name: #ty ))
}

pub(super) fn args(params: &[NysaParam], is_mutable: bool) -> Result<Vec<FnArg>, ParserError> {
    let mut args = params
        .iter()
        .map(parse_parameter)
        .collect::<Result<Vec<_>, _>>()?;
    if is_mutable {
        args.insert(0, parse_quote!(&mut self))
    } else {
        args.insert(0, parse_quote!(&self))
    }
    Ok(args)
}

pub(super) fn parse_ret_type(returns: &[NysaExpression]) -> Result<syn::ReturnType, ParserError> {
    Ok(match returns.len() {
        0 => parse_quote!(),
        1 => {
            let param = returns.get(0).unwrap().clone();
            let ty = ty::parse_plain_type_from_expr(&param)?;
            parse_quote!(-> #ty)
        }
        _ => {
            let types = returns
                .iter()
                .map(|ret| ty::parse_plain_type_from_expr(ret))
                .collect::<Result<syn::punctuated::Punctuated<syn::Type, syn::Token![,]>, _>>()?;
            parse_quote!(-> (#types))
        }
    })
}

pub(super) fn parse_statements(statements: &[NysaStmt], ctx: &mut Context) -> Vec<syn::Stmt> {
    statements
        .iter()
        .map(|stmt| stmt::parse_statement(&stmt, ctx))
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>()
}

pub(super) fn parse_external_contract_statements(
    params: &[NysaParam],
    ctx: &mut Context,
) -> Vec<syn::Stmt> {
    params
        .iter()
        .filter_map(|param| match &param.ty {
            NysaType::Contract(contract_name) => Some((contract_name, &param.name)),
            _ => None,
        })
        .map(|(contract_name, param_name)| {
            let ident = quote::format_ident!("{}", param_name);
            stmt::parse_ext_contract_stmt(contract_name, ident.clone(), ident, ctx)
        })
        .collect::<Vec<_>>()
}
