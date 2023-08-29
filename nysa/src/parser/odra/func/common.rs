use syn::{parse_quote, FnArg};

use crate::{
    model::ir::{NysaExpression, NysaParam, NysaStmt, NysaType, NysaVisibility},
    parser::odra::{context::Context, stmt, ty},
    utils,
};

pub(super) fn parse_visibility(vis: &NysaVisibility) -> syn::Visibility {
    match vis {
        NysaVisibility::Private => parse_quote!(),
        NysaVisibility::Public => parse_quote!(pub),
    }
}

pub(super) fn parse_parameter(param: &NysaParam) -> syn::FnArg {
    let ty = ty::parse_plain_type_from_ty(&param.ty);
    let name = utils::to_snake_case_ident(&param.name);
    parse_quote!( #name: #ty )
}

pub(super) fn args(params: &[NysaParam], is_mutable: bool) -> Vec<FnArg> {
    let mut args: Vec<FnArg> = params.iter().map(parse_parameter).collect();
    if is_mutable {
        args.insert(0, parse_quote!(&mut self))
    } else {
        args.insert(0, parse_quote!(&self))
    }
    args
}

pub(super) fn parse_ret_type(returns: &[NysaExpression]) -> syn::ReturnType {
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
