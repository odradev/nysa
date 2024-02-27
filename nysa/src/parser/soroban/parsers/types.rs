use crate::{
    error::ParserResult,
    model::ir::{eval_expression_type, Expression, Type},
    parser::{
        common::{NumberParser, StatementParserContext, StringParser, TypeParser},
        context::TypeInfo,
        soroban::code,
    },
    ParserError, SorobanParser,
};

impl NumberParser for SorobanParser {
    fn parse_typed_number<T: StatementParserContext>(
        values: &[u64],
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        parse_typed_number::<_, Self>(values, Type::Uint(256), ctx)
    }

    fn parse_generic_number(expr: &Expression) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn unsigned_one() -> syn::Expr {
        todo!()
    }

    fn words_to_number(words: Vec<u64>, ty: &syn::Type) -> syn::Expr {
        todo!()
    }
}

impl StringParser for SorobanParser {
    fn parse_string(string: &str) -> ParserResult<syn::Expr> {
        Ok(code::expr::string_from(string))
    }
}

impl TypeParser for SorobanParser {
    fn parse_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type> {
        match ty {
            Type::Address => Ok(code::ty::option(code::ty::address())),
            Type::Bool => Ok(code::ty::bool()),
            Type::String => Ok(code::ty::string()),
            Type::Int(size) => Ok(code::ty::int(*size)),
            Type::Uint(size) => Ok(code::ty::uint(*size)),
            Type::Bytes(_) => Ok(code::ty::bytes()),
            Type::Mapping(_, _) => Err(ParserError::InvalidType),
            Type::Custom(_) => todo!(),
            Type::Array(_) => todo!(),
            Type::Unknown => todo!(),
        }
    }

    fn parse_fixed_bytes(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_serialize(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_state_ty<T: TypeInfo>(ty: &Type, ctx: &T) -> ParserResult<syn::Type> {
        todo!()
    }
}

pub(super) fn parse_typed_number<T: StatementParserContext, P: TypeParser>(
    values: &[u64],
    default_ty: Type,
    ctx: &mut T,
) -> ParserResult<syn::Expr> {
    let ty = ctx
        .contextual_expr()
        .map(|e| eval_expression_type(e, ctx))
        .flatten()
        .unwrap_or(default_ty);

    let syn_ty = P::parse_ty(&ty, ctx).unwrap_or(code::ty::u256());

    if let Type::Int(size) | Type::Uint(size) = ty {
        if size > 128 {
            Ok(code::expr::u256_from_le_bytes(values))
        } else {
            Ok(code::expr::from_le_bytes(syn_ty, size, values))
        }
    } else {
        panic!("Invalid type for number literal")
    }
}
