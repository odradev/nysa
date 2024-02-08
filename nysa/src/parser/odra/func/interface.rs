use syn::parse_quote;

use crate::{
    error::ParserResult, model::ir::Function, parser::context::TypeInfo, utils, ParserError,
};

use super::common;

pub fn def<T: TypeInfo>(f: &Function, ctx: &T) -> ParserResult<syn::TraitItem> {
    if let Function::Function(function) = f {
        let args = common::args(&function.params, function.is_mutable, ctx)?;
        let ret = common::parse_ret_type(&function.ret, ctx)?;
        let ident = utils::to_snake_case_ident(&function.name);

        Ok(parse_quote!(fn #ident( #(#args),* ) #ret;))
    } else {
        Err(ParserError::InvalidFunctionType(
            "NysaFunction::Function",
            f.clone(),
        ))
    }
}
