use crate::error::ParserResult;
use crate::model::ir::{eval_expression_type, Expression, Type};
use crate::parser::common::{expr, StatementParserContext};
use crate::parser::context::{ContractInfo, FnContext, TypeInfo};
use crate::{utils, Parser};

use crate::parser::syn_utils;

/// A variable declaration. Creates a syn::Stmt which creates a mutable variable with a given
/// name assigning its default value.
///
/// Updates the context - registers a local variables.
///
/// ## Solidity example
/// `uint128 liquidityNext;`
///
/// ## Arguments
/// * name - variable name
/// * ty - variable type
/// * ctx - parser context
pub fn declaration<T>(name: &str, ty: &Type, ctx: &mut T) -> ParserResult<syn::Stmt>
where
    T: FnContext,
{
    let name = utils::to_snake_case_ident(name);
    ctx.register_local_var(&name, ty);
    Ok(syn_utils::definition(name, syn_utils::default()))
}

/// A variable definition. Creates a syn::Stmt which creates a mutable variable with a given
/// name and a given value.
///
/// Updates the context - registers a local variables.
///
/// ## Solidity example
/// `uint128 liquidityNext = 1234;`
///
/// ## Arguments
/// * name - variable name
/// * ty - variable type
/// * init - an expression that initializes the variable
/// * ctx - parser context
pub fn definition<T, P>(
    name: &str,
    ty: &Type,
    init: &Expression,
    ctx: &mut T,
) -> ParserResult<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    let name = utils::to_snake_case_ident(name);
    let expr = expr::var::parse_or_default::<_, P>(init, ctx)?;
    register_var(&name, ty, init, ctx);
    Ok(syn_utils::definition(name, expr))
}

fn register_var<T, S>(name: S, ty: &Type, init: &Expression, ctx: &mut T)
where
    T: TypeInfo + ContractInfo + FnContext,
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
    use crate::{model::ir::Stmt, parser::test_utils::{assert_tokens_eq, unsafe_parse_with_empty_context}};
    use quote::quote;
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
