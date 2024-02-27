use self::syn_utils::{ArrayReader, DefaultValue, ReadValue, UnwrapOrNone, UnwrapOrRevert};
use crate::error::ParserResult;
use crate::model::ir::{Expression, MathOp, Type};
use crate::parser::common::expr::math::eval_in_context;
use crate::parser::common::{ExpressionParser, StatementParserContext, StringParser};
use crate::parser::context::{ItemType, StorageInfo, TypeInfo};
use crate::parser::syn_utils::{AsExpression, AsSelfField};
use crate::{formatted_invalid_expr, utils, OdraParser};
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Token};

pub(crate) mod error;
mod num;
pub(crate) mod syn_utils;

impl StringParser for OdraParser {
    fn parse_string(string: &str) -> ParserResult<syn::Expr> {
        Ok(syn_utils::string_from(string))
    }
}

impl ExpressionParser for OdraParser {
    fn parse_set_storage_expression(name: &str, value_expr: syn::Expr) -> ParserResult<syn::Expr> {
        let ident = utils::to_snake_case_ident(name);
        Ok(parse_quote!(self.#ident.set(#value_expr)))
    }

    fn parse_storage_value_expression<T: StorageInfo + TypeInfo>(
        name: &str,
        key_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let field = crate::utils::to_snake_case_ident(name).as_self_field();

        // let field = ident.as_self_field();

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

                Self::parse_storage_value_expression(name, key_expr, &ty, ctx)
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
                Self::parse_storage_value_expression(
                    var_ident.to_string().as_str(),
                    Some(key),
                    ty,
                    ctx,
                )
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

        match value_expr {
            Some(value) => {
                let field = var_ident.as_self_field();
                Ok(parse_quote!(#field.set(&#key, #value)))
            }
            None => Self::parse_storage_value_expression(
                var_ident.to_string().as_str(),
                Some(key),
                ty,
                ctx,
            ),
        }
    }

    fn parse_var_type(name: Ident, item_type: &Option<ItemType>) -> ParserResult<syn::Expr> {
        match item_type {
            // Variable update must use the `set` function
            Some(ItemType::Storage(_)) => Ok(name.as_self_field()),
            // regular, local value
            Some(ItemType::Local(_)) => Ok(name.as_expression()),
            None => Ok(name.as_expression()),
            _ => formatted_invalid_expr!("unknown variable {:?}", item_type),
        }
    }

    fn caller() -> syn::Expr {
        parse_quote!(self.env().caller())
    }

    fn parse_math_op<T: StatementParserContext>(
        left: &crate::model::ir::Expression,
        right: &crate::model::ir::Expression,
        op: &crate::model::ir::MathOp,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        if *op == MathOp::Pow {
            return pow(left, right, ctx);
        }
        let op: syn::BinOp = op.into();
        let left_expr = eval_in_context::<_, OdraParser>(left, right, ctx)?;
        let right_expr = eval_in_context::<_, OdraParser>(right, left, ctx)?;
        Ok(parse_quote!( (#left_expr #op #right_expr) ))
    }

    fn parse_update_value_expression<T: StatementParserContext>(
        current_value: syn::Expr,
        value: syn::Expr,
        op: syn::BinOp,
        ty: Type,
        ctx: &mut T,
    ) -> syn::Expr {
        parse_quote!(#current_value #op #value)
    }
}

fn pow<T: StatementParserContext>(
    left: &Expression,
    right: &Expression,
    ctx: &mut T,
) -> ParserResult<::syn::Expr> {
    let left_expr = eval_in_context::<_, OdraParser>(left, right, ctx)?;
    let right_expr = eval_in_context::<_, OdraParser>(right, left, ctx)?;
    Ok(parse_quote!(#left_expr.pow(#right_expr)))
}
