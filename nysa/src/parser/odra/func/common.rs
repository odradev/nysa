use syn::{parse_quote, FnArg};

use crate::error::ParserResult;
use crate::parser::common::StatementParserContext;
use crate::parser::context::ItemType;
use crate::{
    model::ir::{Expression, Param, Stmt, Type, Visibility},
    parser::{
        context::{ContractInfo, ExternalCallsRegister, FnContext, TypeInfo},
        odra::{stmt, ty},
        syn_utils,
    },
    utils,
};
use crate::{parser, OdraParser};

pub(super) fn parse_visibility(vis: &Visibility) -> syn::Visibility {
    match vis {
        Visibility::Private => parse_quote!(),
        Visibility::Public => parse_quote!(pub),
        Visibility::Internal => parse_quote!(pub(crate)),
    }
}

pub(super) fn parse_parameter<T: TypeInfo>(param: &Param, info: &T) -> ParserResult<syn::FnArg> {
    let ty = ty::parse_type_from_ty(&param.ty, info)?;
    let name = utils::to_snake_case_ident(&param.name);
    Ok(syn_utils::fn_arg(name, ty))
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
    args.insert(0, syn_utils::self_arg(is_mutable));

    params
        .iter()
        .for_each(|p| ctx.register_local_var(&p.name, &p.ty));

    Ok(args)
}

pub(super) fn args<T: TypeInfo>(
    params: &[Param],
    is_mutable: bool,
    ctx: &T,
) -> ParserResult<Vec<FnArg>> {
    let mut args = params
        .iter()
        .map(|p| parse_parameter(p, ctx))
        .collect::<ParserResult<Vec<_>>>()?;
    args.insert(0, syn_utils::self_arg(is_mutable));

    Ok(args)
}

pub(super) fn parse_ret_type<T: TypeInfo>(
    returns: &[(Option<String>, Expression)],
    ctx: &T,
) -> ParserResult<syn::ReturnType> {
    Ok(match returns.len() {
        0 => parse_quote!(),
        1 => {
            let (_, e) = returns.get(0).unwrap().clone();
            let ty = ty::parse_type_from_expr(&e, ctx)?;
            parse_quote!(-> #ty)
        }
        _ => {
            let types = returns
                .iter()
                .map(|(_, e)| ty::parse_type_from_expr(e, ctx))
                .collect::<Result<syn::punctuated::Punctuated<syn::Type, syn::Token![,]>, _>>()?;
            parse_quote!(-> (#types))
        }
    })
}

pub(super) fn parse_statements<T>(statements: &[Stmt], ctx: &mut T) -> Vec<syn::Stmt>
where
    T: StatementParserContext,
{
    statements
        .iter()
        .map(|stmt| stmt::parse_statement(&stmt, true, ctx))
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
                Some(
                    parser::common::stmt::ext::ext_contract_stmt::<_, OdraParser>(
                        param_name,
                        &contract_name,
                        ctx,
                    ),
                )
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
