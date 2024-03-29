use crate::{
    model::ir::Type,
    parser::{
        context::{ExternalCallsRegister, FnContext},
        odra::stmt::syn_utils,
    },
};

/// Builds a syn::Stmt representing an external contract reference declaration.
///
/// # Arguments
/// * `var_name` - The name of the variable that will hold the contract reference.
/// * `contract_name` - The name of the contract to reference.
/// * `ctx` - A mutable reference to the context object that provides information about the contract.
pub fn ext_contract_stmt<R>(var_name: &str, contract_name: &str, ctx: &mut R) -> syn::Stmt
where
    R: ExternalCallsRegister + FnContext,
{
    ctx.register_external_call(contract_name);
    ctx.register_local_var(var_name, &Type::Custom(contract_name.to_string()));

    syn_utils::contract_ref(var_name, contract_name)
}
