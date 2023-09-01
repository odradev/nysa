use syn::parse_quote;

use crate::{model::ir::NysaFunction, parser::context::Context, utils, ParserError};

use super::common;

pub fn def(f: &NysaFunction, ctx: &Context) -> Result<syn::TraitItem, ParserError> {
    if let NysaFunction::Function(function) = f {
        let args = common::args(&function.params, function.is_mutable, ctx)?;
        let ret = common::parse_ret_type(&function.ret, ctx)?;
        let ident = utils::to_snake_case_ident(&function.name);

        Ok(parse_quote!(fn #ident( #(#args),* ) #ret;))
    } else {
        Err(ParserError::InvalidFunctionType(
            String::from("NysaFunction::Function"),
            f.clone(),
        ))
    }
}
