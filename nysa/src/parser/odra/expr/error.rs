use super::{parse, syn_utils};
use crate::{
    error::ParserResult,
    model::ir::Expression,
    parser::context::{
        ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
        TypeInfo,
    },
    utils, ParserError,
};

/// Reverts the execution of the contract with an error message.
///
/// # Arguments
///
/// * `condition` - An optional expression representing a condition. If the condition is false, the contract execution will be reverted.
/// * `error` - The error message to be displayed when reverting.
/// * `ctx` - A mutable reference to the context object that provides information about the contract.
///
/// # Returns
///
/// Returns a `ParserResult` containing a `syn::Expr` representing the revert expression if successful, or a `ParserError` if an error occurs.
pub fn revert<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    condition: Option<&Expression>,
    error: &Expression,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    match error {
        Expression::StringLiteral(message) => revert_with_str(condition, message, ctx),
        _ => Err(ParserError::UnexpectedExpression(
            "Error should be Expression::StringLiteral",
            error.clone(),
        )),
    }
}

/// Reverts the execution of the contract with an error message.
///
/// # Arguments
/// * `condition` - An optional expression representing a condition. If the condition is false, the contract execution will be reverted.
/// * `message` - The error message to be displayed when reverting.
/// * `ctx` - A mutable reference to the context object that provides information about the contract.
///
/// # Returns
/// Returns a `ParserResult` containing a `syn::Expr` representing the revert expression if successful, or a `ParserError` if an error occurs.
pub fn revert_with_str<
    T: StorageInfo
        + TypeInfo
        + EventsRegister
        + ExternalCallsRegister
        + ContractInfo
        + FnContext
        + ErrorInfo,
>(
    condition: Option<&Expression>,
    message: &str,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let error_num = match ctx.get_error(message) {
        Some(value) => value,
        None => {
            ctx.insert_error(message);
            ctx.error_count()
        }
    };

    let error = syn_utils::revert_user_error(error_num);

    match condition {
        Some(condition) => {
            let condition = parse(condition, ctx)?;
            Ok(syn_utils::if_not(condition, error))
        }
        None => Ok(error),
    }
}

/// Reverts the execution of the contract with an error message.
///
/// # Arguments
/// * `error_name` - The error name to revert with.
///
/// # Returns
/// Returns a `syn::Expr` representing the revert expression.
pub fn revert_with_err(error_name: &str) -> syn::Expr {
    let error = utils::to_ident(error_name);
    syn_utils::revert(error)
}
