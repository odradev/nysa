use syn::parse_quote;
use syn::punctuated::Punctuated;

use crate::error::ParserResult;
use crate::parser::common::{
    stmt, ty, ContractReferenceParser, StatementParserContext, TypeParser,
};
use crate::parser::context::ItemType;
use crate::Parser;
use crate::{
    model::ir::{Expression, Param, Stmt, Type, Visibility},
    parser::{
        context::{ContractInfo, ExternalCallsRegister, FnContext, TypeInfo},
        syn_utils,
    },
    utils,
};

pub(super) fn parse_visibility(vis: &Visibility) -> syn::Visibility {
    match vis {
        Visibility::Private => parse_quote!(),
        Visibility::Public => parse_quote!(pub),
        Visibility::Internal => parse_quote!(pub(crate)),
    }
}

pub(super) fn parse_parameter<T: TypeInfo, P: TypeParser>(
    param: &Param,
    info: &T,
) -> ParserResult<syn::FnArg> {
    let ty = P::parse_ty(&param.ty, info)?;
    let name = utils::to_snake_case_ident(&param.name);
    Ok(syn_utils::fn_arg(name, ty))
}

pub(super) fn parse_params<T: TypeInfo, P: Parser>(
    params: &[Param],
    ctx: &T,
) -> ParserResult<Vec<syn::FnArg>> {
    params
        .iter()
        .map(|p| parse_parameter::<_, P::TypeParser>(p, ctx))
        .collect::<ParserResult<Vec<_>>>()
}

pub(super) fn register_local_vars<T: TypeInfo + FnContext>(params: &[Param], ctx: &mut T) {
    params
        .iter()
        .for_each(|p| ctx.register_local_var(&p.name, &p.ty));
}

pub(super) fn parse_ret_type<T: TypeInfo, P: TypeParser>(
    returns: &[(Option<String>, Expression)],
    ctx: &T,
) -> ParserResult<syn::ReturnType> {
    Ok(match returns.len() {
        0 => parse_quote!(),
        1 => {
            let (_, e) = returns.get(0).unwrap().clone();
            let ty = ty::parse_type_from_expr::<_, P>(&e, ctx)?;
            parse_quote!(-> #ty)
        }
        _ => {
            let types = returns
                .iter()
                .map(|(_, e)| ty::parse_type_from_expr::<_, P>(e, ctx))
                .collect::<ParserResult<Punctuated<syn::Type, syn::Token![,]>>>()?;
            parse_quote!(-> (#types))
        }
    })
}

pub(super) fn parse_statements<T, P>(statements: &[Stmt], ctx: &mut T) -> Vec<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    statements
        .iter()
        .map(|stmt| stmt::parse_statement::<_, P>(&stmt, true, ctx))
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

pub(super) fn parse_external_contract_statements<
    T: ExternalCallsRegister + ContractInfo + FnContext + TypeInfo,
    P: ContractReferenceParser,
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
                Some(stmt::ext::ext_contract_stmt::<_, P>(
                    param_name,
                    &contract_name,
                    ctx,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
