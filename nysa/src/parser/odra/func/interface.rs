use syn::parse_quote;

use crate::{model::ir::NysaFunction, parser::context::TypeInfo, utils, ParserError};

use super::common;

pub fn def<T: TypeInfo>(f: &NysaFunction, info: &T) -> Result<syn::TraitItem, ParserError> {
    if let NysaFunction::Function(function) = f {
        let args = common::args(&function.params, function.is_mutable, info)?;
        let ret = common::parse_ret_type(&function.ret, info)?;
        let ident = utils::to_snake_case_ident(&function.name);

        Ok(parse_quote!(fn #ident( #(#args),* ) #ret;))
    } else {
        Err(ParserError::InvalidFunctionType(
            String::from("NysaFunction::Function"),
            f.clone(),
        ))
    }
}
