use proc_macro2::Ident;
use syn::parse_quote;

use crate::{
    error::ParserResult,
    model::ir::{Constructor, Func},
    parser::{common::FunctionParser, soroban::code},
    SorobanParser,
};

impl FunctionParser for SorobanParser {
    fn parse_args(
        args: Vec<syn::FnArg>,
        is_mutable: bool,
        uses_sender: bool,
    ) -> ParserResult<Vec<syn::FnArg>> {
        let mut args = args;
        args.insert(0, code::expr::fn_arg::env());
        args.insert(1, code::expr::fn_arg::caller());

        Ok(args)
    }

    fn parse_modifier_args(
        args: Vec<syn::FnArg>,
        is_mutable: bool,
        uses_sender: bool,
    ) -> ParserResult<Vec<syn::FnArg>> {
        let mut args = args;
        args.insert(0, code::expr::fn_arg::env());
        args.insert(1, code::expr::fn_arg::caller());

        Ok(args)
    }

    fn attrs(f: &Func) -> Vec<syn::Attribute> {
        vec![]
    }

    fn constructor_attrs(f: &Constructor) -> Vec<syn::Attribute> {
        vec![]
    }

    fn parse_modifier_call(modifier: Ident, args: Vec<syn::Expr>) -> syn::Stmt {
        let mut args = args;
        args.insert(0, code::expr::cloned_caller());
        args.insert(0, code::expr::cloned_env());

        parse_quote!(Self::#modifier( #(#args),* );)
    }

    fn parse_base_call(base: Ident, args: Vec<syn::Expr>) -> syn::Stmt {
        let mut args = args;
        args.insert(0, code::expr::cloned_caller());
        args.insert(0, code::expr::cloned_env());

        parse_quote!(Self::#base( #(#args),* );)
    }

    fn parse_super_call(fn_name: Ident, args: Vec<syn::Expr>) -> syn::Expr {
        parse_quote!(Self::#fn_name( #(#args),* );)
    }

    fn parse_module_fn_call(fn_name: syn::Expr, args: Vec<syn::Expr>) -> syn::Expr {
        let mut args = args;
        args.insert(0, code::expr::cloned_caller());
        args.insert(0, code::expr::cloned_env());

        parse_quote!(Self::#fn_name(#(#args),*))
    }
}
