use syn::parse_quote;

use crate::model::ir::{eval_expression_type, Expression, Type};
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
    Ok(parse_quote!(let mut #pat = Default::default();))
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
    register_var(&name, ty, init, ctx);
    Ok(parse_quote!(let mut #pat = #expr;))
}

fn register_var<T, S>(name: &S, ty: &Type, init: &Expression, ctx: &mut T)
where
    T: StorageInfo + TypeInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext,
    S: ToString,
{
    if ty == &Type::Unknown {
        let ty = eval_expression_type(init, ctx).unwrap_or(Type::Unknown);
        ctx.register_local_var(name, &ty);
    } else {
        ctx.register_local_var(name, ty);
    }
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

        assert_tokens_eq(
            unsafe_parse_with_empty_context(stmt),
            quote!(let mut x = Default::default();),
        )
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
