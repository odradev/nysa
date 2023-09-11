use quote::{format_ident, ToTokens};
use syn::parse_quote;

use crate::{
    model::ir::Expression,
    parser::{
        context::{
            ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
        },
        odra::{ERRORS, ERROR_MAP},
    },
    ParserError,
};

use super::parse;

pub fn revert<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    condition: Option<&Expression>,
    error: &Expression,
    t: &mut T,
) -> Result<syn::Expr, ParserError> {
    if let Expression::StringLiteral(message) = error {
        revert_with_str(condition, message, t)
    } else {
        Err(ParserError::UnexpectedExpression(
            String::from("Error should be Expression::StringLiteral"),
            error.clone(),
        ))
    }
}

pub fn revert_with_str<
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
>(
    condition: Option<&Expression>,
    message: &str,
    t: &mut T,
) -> Result<syn::Expr, ParserError> {
    let mut error_map = ERROR_MAP.lock().unwrap();
    let mut errors = ERRORS.lock().unwrap();

    let error_num = if let Some(value) = error_map.get(message) {
        value.to_token_stream()
    } else {
        error_map.insert(message.to_string(), *errors + 1);
        *errors += 1;
        errors.to_token_stream()
    };

    let err = quote::quote!(odra::contract_env::revert(odra::types::ExecutionError::new(#error_num, #message)));
    #[cfg(test)]
    let err =
        quote::quote!(odra::contract_env::revert(odra::types::ExecutionError::new(1u16, #message)));

    if let Some(condition) = condition {
        let condition = parse(condition, t)?;
        return Ok(parse_quote!(if !(#condition) { #err }));
    } else {
        return Ok(parse_quote!(#err));
    }
}

pub fn revert_with_err(err: &str) -> Result<syn::Expr, ParserError> {
    let expr = format_ident!("{}", err);
    Ok(parse_quote!(odra::contract_env::revert(Error::#expr)))
}
