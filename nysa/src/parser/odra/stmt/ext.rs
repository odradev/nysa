use crate::{
    model::ir::Type,
    parser::context::{ExternalCallsRegister, FnContext},
};
use quote::{format_ident, ToTokens};
use syn::parse_quote;

pub fn ext_contract_stmt<S: ToTokens, T: ToTokens, R>(
    contract_name: &str,
    ident: S,
    addr: T,
    ctx: &mut R,
) -> syn::Stmt
where
    R: ExternalCallsRegister + FnContext,
{
    ctx.register_external_call(contract_name);
    ctx.register_local_var(
        &ident.to_token_stream(),
        &Type::Custom(contract_name.to_string()),
    );

    let ref_ident = format_ident!("{}Ref", contract_name);
    parse_quote!(let mut #ident = #ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#addr));)
}
