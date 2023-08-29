use core::panic;

use syn::parse_quote;

use crate::{model::ir::NysaFunction, utils};

use super::common;

pub fn def(f: &NysaFunction) -> syn::TraitItem {
    if let NysaFunction::Function(f) = f {
        let args = common::args(&f.params, f.is_mutable);
        let ret = common::parse_ret_type(&f.ret);
        let ident = utils::to_snake_case_ident(&f.name);

        parse_quote!(
            fn #ident( #(#args),* ) #ret;
        )
    } else {
        panic!("Invalid function type")
    }
}
