use crate::parser::context::ExternalCallsRegister;
use quote::{format_ident, ToTokens};
use syn::parse_quote;

pub fn ext_contract_stmt<S: ToTokens, T: ToTokens, R>(
    contract_name: &str,
    ident: S,
    addr: T,
    ctx: &mut R,
) -> syn::Stmt
where
    R: ExternalCallsRegister,
{
    ctx.register_external_call(contract_name);

    let ref_ident = format_ident!("{}Ref", contract_name);
    parse_quote!(let mut #ident = #ref_ident::at(&odra::UnwrapOrRevert::unwrap_or_revert(#addr));)
}
