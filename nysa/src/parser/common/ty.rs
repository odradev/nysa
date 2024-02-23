use syn::parse_quote;

use crate::{
    model::ir::Expression,
    parser::{
        context::{ItemType, TypeInfo},
        syn_utils::AsType,
    },
    utils, ParserError,
};

use super::TypeParser;

pub fn parse_type_from_expr<T: TypeInfo, P: TypeParser>(
    expr: &Expression,
    ctx: &T,
) -> Result<syn::Type, ParserError> {
    let err = || ParserError::UnexpectedExpression("Expression::Type", expr.clone());
    match expr {
        Expression::Type(ty) => P::parse_ty(ty, ctx),
        Expression::MemberAccess(f, box Expression::Variable(name)) => {
            let p = utils::to_snake_case_ident(name);
            let ident = utils::to_ident(f);
            Ok(parse_quote!(#p::#ident))
        }
        Expression::Variable(name) => match ctx.type_from_string(name) {
            Some(ItemType::Enum(_) | ItemType::Struct(_)) => Ok(utils::to_ident(name).as_type()),
            _ => Err(err()),
        },
        _ => Err(err()),
    }
}
