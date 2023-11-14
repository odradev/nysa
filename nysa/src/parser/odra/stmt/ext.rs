use crate::{
    model::ir::Type,
    parser::context::{ExternalCallsRegister, FnContext}, utils,
};
use syn::parse_quote;

pub fn ext_contract_stmt<R>(
    var_name: &str,
    contract_name: &str,
    ctx: &mut R,
) -> syn::Stmt
where
    R: ExternalCallsRegister + FnContext,
{
    ctx.register_external_call(contract_name);
    ctx.register_local_var(
        var_name,
        &Type::Custom(contract_name.to_string()),
    );
    
    let ident = utils::to_ident(var_name);
    let ref_ident = utils::to_ref_ident(contract_name);
    parse_quote!(let mut #ident = #ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#ident));)
}
