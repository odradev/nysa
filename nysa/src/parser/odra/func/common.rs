use syn::{parse_quote, FnArg};

use crate::parser::odra::stmt::ext::ext_contract_stmt;
use crate::{
    model::ir::{Expression, Param, Stmt, Type, Visibility},
    parser::{
        context::{
            ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
        },
        odra::{stmt, ty},
    },
    utils, ParserError,
};

pub(super) fn parse_visibility(vis: &Visibility) -> syn::Visibility {
    match vis {
        Visibility::Private => parse_quote!(),
        Visibility::Public => parse_quote!(pub),
    }
}

pub(super) fn parse_parameter<T: TypeInfo>(
    param: &Param,
    info: &T,
) -> Result<syn::FnArg, ParserError> {
    let ty = ty::parse_plain_type_from_ty(&param.ty, info)?;
    let name = utils::to_snake_case_ident(&param.name);
    Ok(parse_quote!( #name: #ty ))
}

pub(super) fn args<T: TypeInfo>(
    params: &[Param],
    is_mutable: bool,
    info: &T,
) -> Result<Vec<FnArg>, ParserError> {
    let mut args = params
        .iter()
        .map(|p| parse_parameter(p, info))
        .collect::<Result<Vec<_>, _>>()?;
    if is_mutable {
        args.insert(0, parse_quote!(&mut self))
    } else {
        args.insert(0, parse_quote!(&self))
    }
    Ok(args)
}

pub(super) fn parse_ret_type<T: TypeInfo>(
    returns: &[(Option<String>, Expression)],
    ctx: &T,
) -> Result<syn::ReturnType, ParserError> {
    Ok(match returns.len() {
        0 => parse_quote!(),
        1 => {
            let (_, e) = returns.get(0).unwrap().clone();
            let ty = ty::parse_plain_type_from_expr(&e, ctx)?;
            parse_quote!(-> #ty)
        }
        _ => {
            let types = returns
                .iter()
                .map(|(_, e)| ty::parse_plain_type_from_expr(e, ctx))
                .collect::<Result<syn::punctuated::Punctuated<syn::Type, syn::Token![,]>, _>>()?;
            parse_quote!(-> (#types))
        }
    })
}

pub(super) fn parse_statements<T>(statements: &[Stmt], ctx: &mut T) -> Vec<syn::Stmt>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    statements
        .iter()
        .map(|stmt| stmt::parse_statement(&stmt, true, ctx))
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>()
}

pub(super) fn parse_external_contract_statements<T: ExternalCallsRegister + ContractInfo>(
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
            if ctx.is_class(name) {
                let ident = quote::format_ident!("{}", param_name);
                Some(ext_contract_stmt(name, ident.clone(), ident, ctx))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
