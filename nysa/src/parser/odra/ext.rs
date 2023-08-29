use quote::format_ident;
use syn::parse_quote;

use crate::{model::ir::Package, parser::odra::func, utils};

pub(crate) fn errors_ext_contract(package: &Package) -> Vec<syn::ItemMod> {
    let interfaces = package.interfaces();

    interfaces
        .iter()
        .map(|i| {
            let ident = format_ident!("{}", i.contract().name());
            let fns = i
                .fns()
                .iter()
                .map(|f| func::interface::def(f))
                .collect::<Vec<_>>();
            let mod_ident = utils::to_snake_case_ident(i.contract().name());
            parse_quote!(
                pub mod #mod_ident {
                    #[odra::external_contract]
                    pub trait #ident {
                        #(#fns)*
                    }
                }
            )
        })
        .collect()
}
