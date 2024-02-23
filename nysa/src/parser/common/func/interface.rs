use syn::parse_quote;

use crate::{
    error::ParserResult,
    model::ir::{contains_sender_expr, Function},
    parser::{common::FunctionParser, context::TypeInfo},
    utils, Parser, ParserError,
};

use super::common;

pub fn def<T: TypeInfo, P: Parser>(f: &Function, ctx: &T) -> ParserResult<syn::TraitItem> {
    if let Function::Function(function) = f {
        let args = common::parse_params::<_, P>(&function.params, ctx)?;
        let args = <P::FnParser as FunctionParser>::parse_args(
            args,
            function.is_mutable,
            contains_sender_expr(&function.stmts),
        )?;

        let ret = common::parse_ret_type::<_, P::TypeParser>(&function.ret, ctx)?;
        let ident = utils::to_snake_case_ident(&function.name);

        Ok(parse_quote!(fn #ident( #(#args),* ) #ret;))
    } else {
        Err(ParserError::InvalidFunctionType(
            "NysaFunction::Function",
            f.clone(),
        ))
    }
}
