use quote::ToTokens;
use syn::parse_quote;

use crate::{ERRORS, ERROR_MAP};

use super::{values::{NysaExpression, StorageField}, parse};


pub fn revert(
    condition: &NysaExpression,
    error: &NysaExpression,
    storage_fields: &[StorageField],
) -> Result<syn::Expr, &'static str> {
    let mut error_map = ERROR_MAP.lock().unwrap();
    let mut errors = ERRORS.lock().unwrap();

    if let NysaExpression::StringLiteral(message) = error {
        let error_num = if let Some(value) = error_map.get(message) {
            value.to_token_stream()
        } else {
            error_map.insert(message.clone(), *errors + 1);
            *errors += 1;
            errors.to_token_stream()
        };
        let condition = parse(condition, storage_fields)?;

        return Ok(
            parse_quote!(if !(#condition) { odra::contract_env::revert(odra::types::ExecutionError::new(#error_num, #message)) }),
        );
    } else {
        Err("Error should be pt::Expression::StringLiteral")
    }
}
