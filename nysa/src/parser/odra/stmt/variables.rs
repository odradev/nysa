use syn::parse_quote;

use crate::model::ir::{Expression, Type};
use crate::parser::context::{
    ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
};
use crate::parser::odra::expr;
use crate::parser::odra::stmt::ext::ext_contract_stmt;
use crate::{utils, ParserError};

pub(super) fn declaration<T>(name: &str, ty: &Type, ctx: &mut T) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let name = utils::to_snake_case_ident(name);
    let pat: syn::Pat = parse_quote!(#name);
    ctx.register_local_var(&name, ty);
    Ok(parse_quote!(let #pat;))
}

pub(super) fn definition<T>(
    name: &str,
    ty: &Type,
    init: &Expression,
    ctx: &mut T,
) -> Result<syn::Stmt, ParserError>
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
{
    let name = utils::to_snake_case_ident(name);
    let pat: syn::Pat = parse_quote! { #name };
    if let Expression::Func(name, args) = init {
        if let Some(class_name) = ctx.as_contract_name(name) {
            let args = expr::parse_many(&args, ctx)?;
            let addr = args.get(0);
            return Ok(ext_contract_stmt(&class_name, pat, addr, ctx));
        }
    };
    let expr: syn::Expr = expr::primitives::get_var_or_parse(init, ctx)?;
    ctx.register_local_var(&name, ty);
    Ok(parse_quote!(let mut #pat = #expr;))
}

#[cfg(test)]
mod tests {
    use crate::model::ir::Stmt;
    use quote::quote;

    use crate::parser::odra::stmt::test::unsafe_parse_with_empty_context;
    use crate::parser::odra::test::assert_tokens_eq;

    use super::*;

    #[test]
    fn var_declaration() {
        let stmt = Stmt::VarDeclaration("x".to_string(), Type::Bool);

        assert_tokens_eq(unsafe_parse_with_empty_context(stmt), quote!(let x;))
    }

    #[test]
    fn var_definition() {
        let stmt = Stmt::VarDefinition(
            "my_var".to_string(),
            Type::Bool,
            Expression::BoolLiteral(false),
        );

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(let mut my_var = false;),
        )
    }
}
