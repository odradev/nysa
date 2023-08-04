use quote::ToTokens;
use solidity_parser::pt;
use syn::parse_quote;

use crate::{ERRORS, ERROR_MAP};

use super::parse;

pub fn revert(
    condition: &pt::Expression,
    error: &pt::Expression,
    storage_fields: &[&pt::VariableDefinition],
) -> Result<syn::Expr, &'static str> {
    let mut error_map = ERROR_MAP.lock().unwrap();
    let mut errors = ERRORS.lock().unwrap();

    if let pt::Expression::StringLiteral(strings) = error {
        let strings = strings
            .iter()
            .map(|lit| lit.string.clone())
            .collect::<Vec<_>>();
        let error_message = strings.join(",");

        let error_num = if let Some(value) = error_map.get(&error_message) {
            value.to_token_stream()
        } else {
            error_map.insert(error_message.clone(), *errors + 1);
            *errors += 1;
            errors.to_token_stream()
        };
        let condition = parse(condition, storage_fields)?;

        return Ok(
            parse_quote!(if !(#condition) { odra::contract_env::revert(odra::types::ExecutionError::new(#error_num, #error_message)) }),
        );
    } else {
        Err("Error should be pt::Expression::StringLiteral")
    }
}
