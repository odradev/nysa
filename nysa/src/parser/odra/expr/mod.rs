use self::syn_utils::{ArrayReader, DefaultValue, ReadValue, UnwrapOrNone, UnwrapOrRevert};
use crate::error::ParserResult;
use crate::model::ir::{Expression, Type};
use crate::parser::common::{ExpressionParser, StatementParserContext, StringParser, TypeParser};
use crate::parser::context::{ItemType, TypeInfo};
use crate::parser::syn_utils::AsSelfField;
use crate::{parser, OdraParser, Parser};
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Token};

pub(crate) mod error;
mod num;
pub(crate) mod primitives;
pub(crate) mod syn_utils;

pub fn parse<T, P>(expression: &Expression, ctx: &mut T) -> ParserResult<syn::Expr>
where
    T: StatementParserContext,
    P: Parser,
{
    parser::common::expr::parse::<_, P>(expression, ctx)
}

impl StringParser for OdraParser {
    fn parse_string(string: &str) -> ParserResult<syn::Expr> {
        Ok(syn_utils::string_from(string))
    }
}

impl ExpressionParser for OdraParser {
    fn parse_set_var_expression(
        var_expr: syn::Expr,
        value_expr: syn::Expr,
        item_type: Option<ItemType>,
    ) -> ParserResult<syn::Expr> {
        Ok(match item_type {
            // Variable update must use the `set` function
            Some(ItemType::Storage(_)) => parse_quote!(#var_expr.set(#value_expr)),
            _ => parse_quote!(#var_expr = #value_expr),
        })
    }

    fn parse_read_values_expression<
        F: quote::ToTokens,
        T: parser::context::StorageInfo + TypeInfo,
    >(
        field: F,
        key_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> syn::Expr {
        let key = key_expr.clone().map(|k| quote!(&#k));
        match ty {
            Type::Address => <UnwrapOrNone as ReadValue>::expr(field, key),
            Type::Custom(name) => ctx
                .type_from_string(&name)
                .map(|ty| match ty {
                    ItemType::Contract(_) | ItemType::Library(_) | ItemType::Interface(_) => {
                        <UnwrapOrNone as ReadValue>::expr(field, key)
                    }
                    ItemType::Enum(_) => <DefaultValue as ReadValue>::expr(field, key),
                    _ => <UnwrapOrRevert as ReadValue>::expr(field, key),
                })
                .unwrap(),
            Type::String | Type::Bool | Type::Uint(_) | Type::Int(_) => {
                <DefaultValue as ReadValue>::expr(field, key)
            }
            Type::Mapping(_, v) => {
                let ty = match ctx.type_from_expression(v) {
                    Some(ItemType::Struct(s)) => Type::Custom(s.name),
                    _ => Type::try_from(&**v).unwrap(),
                };

                Self::parse_read_values_expression(field, key_expr, &ty, ctx)
            }
            Type::Array(ty) => {
                let key = key_expr.and_then(|key| match &**ty {
                    Type::Uint(size) => match size {
                        256..=512 => Some(quote!([#key.as_usize()])),
                        _ => Some(quote!([#key as usize])),
                    },
                    _ => Some(quote!([#key])),
                });
                <ArrayReader as ReadValue>::expr(field, key)
            }
            _ => <UnwrapOrRevert as ReadValue>::expr(field, key),
        }
    }

    fn parse_local_collection<T: StatementParserContext>(
        var_ident: Ident,
        keys_expr: Vec<syn::Expr>,
        value_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let keys_len = keys_expr.len();
        // A local mapping should not exists but eg. can be passed by a reference to a function.
        if let Type::Mapping(_, _) = ty {
            // iterate over nested mappings
            let key = match keys_expr.len() {
                1 => keys_expr[0].clone(),
                _ => {
                    let keys = keys_expr
                        .into_iter()
                        .collect::<Punctuated<syn::Expr, Token![,]>>();
                    parse_quote!( ((#keys),*) )
                }
            };
            return if let Some(value) = value_expr {
                Ok(parse_quote!(#var_ident.set(&#key, #value)))
            } else {
                Ok(Self::parse_read_values_expression(
                    var_ident,
                    Some(key),
                    ty,
                    ctx,
                ))
            };
        }

        let mut collection = quote!(#var_ident);
        for k in keys_expr {
            collection.extend(quote!([#k]));
        }

        let assign = value_expr.map(|e| quote!(= #e));
        Ok(parse_quote!(#collection #assign))
    }

    fn parse_storage_collection<T: StatementParserContext>(
        var_ident: Ident,
        keys_expr: Vec<syn::Expr>,
        value_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let key = match keys_expr.len() {
            1 => keys_expr[0].clone(),
            _ => {
                let keys = keys_expr
                    .into_iter()
                    .collect::<Punctuated<syn::Expr, Token![,]>>();
                parse_quote!((#keys))
            }
        };

        let field = var_ident.as_self_field();
        match value_expr {
            Some(value) => Ok(parse_quote!(#field.set(&#key, #value))),
            None => Ok(Self::parse_read_values_expression(
                field,
                Some(key),
                ty,
                ctx,
            )),
        }
    }
}

impl TypeParser for OdraParser {
    fn parse_ty<T: StatementParserContext>(ty: &Type, ctx: &mut T) -> ParserResult<syn::Type> {
        super::ty::parse_type_from_ty(ty, ctx)
    }

    fn parse_fixed_bytes(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        Ok(syn_utils::try_fixed_bytes(&args))
    }

    fn parse_serialize(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        Ok(syn_utils::serialize(&args))
    }
}
