use crate::{
    error::ParserResult,
    parser::common::{NumberParser, StatementParserContext},
};

pub(super) fn to_typed_int_expr<T: StatementParserContext, P: NumberParser>(
    values: &[u64],
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    P::parse_typed_number(values, ctx)
}

pub(crate) fn to_generic_lit_expr<N: num_traits::Num + ToString>(num: N) -> syn::Expr {
    syn::Expr::Lit(syn::ExprLit {
        attrs: vec![],
        lit: syn::Lit::Int(syn::LitInt::new(
            &num.to_string(),
            proc_macro2::Span::call_site(),
        )),
    })
}
