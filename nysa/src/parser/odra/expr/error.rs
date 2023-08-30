use quote::{format_ident, ToTokens};
use syn::parse_quote;

use crate::{
    model::ir::NysaExpression,
    parser::{
        context::Context,
        odra::{ERRORS, ERROR_MAP},
    },
    ParserError,
};

use super::parse;

pub fn revert(
    condition: Option<&NysaExpression>,
    error: &NysaExpression,
    ctx: &mut Context,
) -> Result<syn::Expr, ParserError> {
    if let NysaExpression::StringLiteral(message) = error {
        revert_with_str(condition, message, ctx)
    } else {
        Err(ParserError::UnexpectedExpression(
            String::from("Error should be NysaExpression::StringLiteral"),
            error.clone(),
        ))
    }
}

pub fn revert_with_str(
    condition: Option<&NysaExpression>,
    message: &str,
    ctx: &mut Context,
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
        let condition = parse(condition, ctx)?;
        return Ok(parse_quote!(if !(#condition) { #err }));
    } else {
        return Ok(parse_quote!(#err));
    }
}

pub fn revert_with_err(err: &str) -> Result<syn::Expr, ParserError> {
    let expr = format_ident!("{}", err);
    Ok(parse_quote!(odra::contract_env::revert(Error::#expr)))
}
