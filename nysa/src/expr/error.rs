use quote::ToTokens;
use syn::parse_quote;

use crate::{
    model::{NysaExpression, StorageField},
    ERRORS, ERROR_MAP,
};

use super::parse;

pub fn revert(
    condition: Option<&NysaExpression>,
    error: &NysaExpression,
    storage_fields: &[StorageField],
) -> Result<syn::Expr, &'static str> {
    if let NysaExpression::StringLiteral(message) = error {
        revert_with_str(condition, message, storage_fields)
    } else {
        Err("Error should be pt::Expression::StringLiteral")
    }
}

pub fn revert_with_str(
    condition: Option<&NysaExpression>,
    message: &str,
    storage_fields: &[StorageField],
) -> Result<syn::Expr, &'static str> {
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
    let err = quote::quote!(odra::contract_env::revert(odra::types::ExecutionError::new(1u16, #message)));

    if let Some(condition) = condition {
        let condition = parse(condition, storage_fields)?;
        return Ok(
            parse_quote!(if !(#condition) { #err }),
        );
    } else {
        return Ok(
            parse_quote!(#err),
        );
    }
}
