use proc_macro2::{Ident, TokenStream};
use syn::{punctuated::Punctuated, FnArg, Token};

use super::context::{
    ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, ItemType,
    StorageInfo, TypeInfo,
};
use crate::{
    error::ParserResult,
    model::ir::{Constructor, Enum, Expression, Func, MathOp, Struct, Type},
};

pub(crate) mod custom;
pub(crate) mod errors;
pub(crate) mod event;
pub(crate) mod expr;
pub(crate) mod ext;
pub(crate) mod func;
pub(crate) mod other;
pub(crate) mod stmt;
pub(super) mod ty;
pub(crate) mod var;

pub trait StatementParserContext:
    StorageInfo
    + TypeInfo
    + EventsRegister
    + ExternalCallsRegister
    + ContractInfo
    + FnContext
    + ErrorInfo
{
}

pub trait ContractReferenceParser {
    /// Parses a contract reference into a `syn::Stmt`.
    ///
    /// # Arguments
    /// * `variable_name` - The name of the variable to assign the contract reference to.
    /// * `contract_name` - The name of the contract to reference.
    ///
    /// # Returns
    /// Returns a `syn::Stmt` representing the contract reference. The statement will be an assignment of
    /// the contract reference to the variable.
    fn parse_contract_ref(variable_name: &str, contract_name: &str) -> syn::Stmt;
    /// Parses a contract reference into a `syn::Expr`.
    ///
    /// # Arguments
    /// * `contract_name` - The name of the contract to reference.
    /// * `address` - An expression representing address of the contract.
    ///
    /// # Returns
    /// Returns a `syn::Expr` representing the contract reference.
    fn parse_contract_ref_expr(contract_name: &str, address: syn::Expr) -> syn::Expr;
}

pub trait EventEmitParser {
    /// Parses an emit statement into a `syn::Stmt`.
    ///
    /// # Arguments
    /// * `event_ident` - The identifier of the event to emit.
    /// * `args` - The arguments to pass to the event.
    ///
    /// # Returns
    /// Returns a `ParserResult` containing a `syn::Stmt` representing the emit statement if successful,
    /// or a `ParserError` if an error occurs.
    fn parse_emit_stmt(event_ident: Ident, args: Vec<syn::Expr>) -> ParserResult<syn::Stmt>;
}

pub trait ContractErrorParser {
    // fn revert_with_msg(msg: &str) -> ParserResult<syn::Stmt>;
    // fn revert<T: StatementParserContext>(condition: Option<&Expression>, msg: &Expression, ctx: &mut T) -> ParserResult<syn::Stmt>;
    /// Reverts the execution of the contract with an error message.
    ///
    /// # Arguments
    /// * `condition` - An optional expression representing a condition. If the condition is false, the contract execution will be reverted.
    /// * `message` - The error message to be displayed when reverting.
    /// * `ctx` - A mutable reference to the context object that provides information about the contract.
    ///
    /// # Returns
    /// Returns a `ParserResult` containing a `syn::Expr` representing the revert expression if successful, or a `ParserError` if an error occurs.
    fn revert_with_str<T: StatementParserContext>(
        condition: Option<&Expression>,
        message: &str,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr>;
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
    fn revert<T: StatementParserContext>(
        condition: Option<&Expression>,
        error: &Expression,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr>;
    /// Reverts the execution of the contract with an error message.
    ///
    /// # Arguments
    /// * `error_name` - The error name to revert with.
    ///
    /// # Returns
    /// Returns a `syn::Expr` representing the revert expression.
    fn revert_with_err(error_name: &str) -> syn::Expr;
}

pub trait ExpressionParser {
    fn parse_set_storage_expression(name: &str, value_expr: syn::Expr) -> ParserResult<syn::Expr>;
    fn parse_storage_value_expression<T: StorageInfo + TypeInfo>(
        name: &str,
        key_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr>;
    fn parse_local_collection<T: StatementParserContext>(
        var_ident: Ident,
        keys_expr: Vec<syn::Expr>,
        value_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr>;
    fn parse_storage_collection<T: StatementParserContext>(
        var_ident: Ident,
        keys_expr: Vec<syn::Expr>,
        value_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr>;
    fn parse_var_type(name: Ident, item_type: &Option<ItemType>) -> ParserResult<syn::Expr>;
    fn caller() -> syn::Expr;

    fn parse_math_op<T: StatementParserContext>(
        left: &Expression,
        right: &Expression,
        op: &MathOp,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr>;
}

pub trait NumberParser {
    /// Parses a typed number expression into a `syn::Expr`.
    fn parse_typed_number<T: StatementParserContext>(
        values: &[u64],
        ctx: &mut T,
    ) -> ParserResult<syn::Expr>;
    /// Parses a generic number expression into a `syn::Expr`.
    fn parse_generic_number(expr: &Expression) -> ParserResult<syn::Expr>;
    /// Returns a `syn::Expr`representing the value of the number `1`.
    fn unsigned_one() -> syn::Expr;
    fn words_to_number(words: Vec<u64>, ty: &syn::Type) -> syn::Expr;
}

pub trait StringParser {
    /// Parses a string into a `syn::Expr`.
    fn parse_string(string: &str) -> ParserResult<syn::Expr>;
}

pub trait TypeParser {
    fn parse_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type>;
    fn parse_state_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type>;
    fn parse_fixed_bytes(args: Vec<syn::Expr>) -> ParserResult<syn::Expr>;
    fn parse_serialize(args: Vec<syn::Expr>) -> ParserResult<syn::Expr>;
}

pub trait FunctionParser {
    fn parse_args(
        args: Vec<syn::FnArg>,
        is_mutable: bool,
        uses_sender: bool,
    ) -> ParserResult<Vec<FnArg>>;

    fn parse_modifier_args(
        args: Vec<syn::FnArg>,
        is_mutable: bool,
        uses_sender: bool,
    ) -> ParserResult<Vec<FnArg>>;

    fn attrs(f: &Func) -> Vec<syn::Attribute>;

    fn constructor_attrs(f: &Constructor) -> Vec<syn::Attribute>;

    fn parse_modifier_call(modifier: Ident, args: Vec<syn::Expr>) -> syn::Stmt;

    fn parse_base_call(base: Ident, args: Vec<syn::Expr>) -> syn::Stmt;

    fn parse_super_call(fn_name: Ident, args: Vec<syn::Expr>) -> syn::Expr;

    fn parse_module_fn_call(fn_name: syn::Expr, args: Vec<syn::Expr>) -> syn::Expr;
}

pub trait CustomElementParser {
    fn parse_custom_struct<T: TypeInfo>(
        namespace: &Option<Ident>,
        model: &Struct,
        ctx: &T,
    ) -> ParserResult<syn::Item>;
    fn parse_custom_enum(name: Ident, model: &Enum) -> syn::Item;
}

pub trait ErrorParser {
    fn errors_def(variants: Punctuated<TokenStream, Token![,]>) -> Option<syn::Item>;
}

pub trait EventParser {
    fn derive_attrs() -> Vec<syn::Attribute>;
}

pub trait ExtContractParser {
    fn parse_ext_contract(
        mod_ident: Ident,
        contract_ident: Ident,
        items: Vec<syn::TraitItem>,
    ) -> ParserResult<syn::ItemMod>;
}

pub trait CustomImports {
    fn import_items() -> Vec<syn::Item>;
}
