use crate::{
    error::ParserResult,
    model::ir::{Constructor, Func},
    parser::{common::FunctionParser, syn_utils},
    OdraParser,
};
use proc_macro2::Ident;
use syn::parse_quote;

impl FunctionParser for OdraParser {
    fn parse_args(
        args: Vec<syn::FnArg>,
        is_mutable: bool,
        uses_sender: bool,
    ) -> ParserResult<Vec<syn::FnArg>> {
        let mut args = args;
        args.insert(0, syn_utils::self_arg(is_mutable));

        Ok(args)
    }

    fn parse_modifier_args(
        args: Vec<syn::FnArg>,
        is_mutable: bool,
        uses_sender: bool,
    ) -> ParserResult<Vec<syn::FnArg>> {
        let mut args = args;
        args.insert(0, syn_utils::self_arg(is_mutable));

        Ok(args)
    }

    fn attrs(f: &Func) -> Vec<syn::Attribute> {
        if f.is_payable {
            vec![super::syn_utils::attr::payable()]
        } else {
            vec![]
        }
    }

    fn constructor_attrs(f: &Constructor) -> Vec<syn::Attribute> {
        if f.is_payable {
            vec![super::syn_utils::attr::payable()]
        } else {
            vec![]
        }
    }

    fn parse_modifier_call(modifier: Ident, args: Vec<syn::Expr>) -> syn::Stmt {
        parse_quote!(self.#modifier( #(#args),* );)
    }

    fn parse_base_call(base: Ident, args: Vec<syn::Expr>) -> syn::Stmt {
        parse_quote!(self.#base( #(#args),* );)
    }

    fn parse_super_call(fn_name: Ident, args: Vec<syn::Expr>) -> syn::Expr {
        parse_quote!(self.#fn_name(#(#args),*))
    }

    fn parse_module_fn_call(fn_name: syn::Expr, args: Vec<syn::Expr>) -> syn::Expr {
        parse_quote!(self.#fn_name(#(#args),*))
    }
}
