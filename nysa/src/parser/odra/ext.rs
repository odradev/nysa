use quote::format_ident;
use syn::parse_quote;

use crate::{model::ir::Package, parser::odra::func, utils, ParserError};

pub(crate) fn errors_ext_contract(package: &Package) -> Result<Vec<syn::ItemMod>, ParserError> {
    let interfaces = package.interfaces();

    interfaces
        .iter()
        .map(|i| {
            let ident = format_ident!("{}", i.contract().name());
            let fns: Vec<syn::TraitItem> = i
                .fns()
                .iter()
                .map(|f| func::interface::def(f))
                .collect::<Result<Vec<_>, _>>()?;

            let mod_ident = utils::to_snake_case_ident(i.contract().name());
            Ok(parse_quote!(
                pub mod #mod_ident {
                    #[odra::external_contract]
                    pub trait #ident {
                        #(#fns)*
                    }
                }
            ))
        })
        .collect()
}
