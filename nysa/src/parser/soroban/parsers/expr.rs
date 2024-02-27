use proc_macro2::Ident;
use syn::parse_quote;

use crate::{
    error::ParserResult,
    formatted_invalid_expr,
    model::ir::{eval_expression_type, Expression, MathOp, Type},
    parser::{
        common::{
            expr::{math::eval_in_context, var},
            ExpressionParser, StatementParserContext, TypeParser,
        },
        context::{ItemType, StorageInfo, TypeInfo},
        soroban::code,
        syn_utils,
    },
    utils, SorobanParser,
};

impl ExpressionParser for SorobanParser {
    fn parse_set_storage_expression(name: &str, value_expr: syn::Expr) -> ParserResult<syn::Expr> {
        let storage = code::expr::storage_instance();
        let key = utils::to_upper_snake_case_ident(name);
        Ok(parse_quote!(#storage.set(&#key, &#value_expr)))
    }

    fn parse_storage_value_expression<T: StorageInfo + TypeInfo>(
        name: &str,
        key_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let key = syn_utils::as_ref(key_expr.clone());

        if let Type::Mapping(k, v) = ty {
            let ty = match ctx.type_from_expression(v) {
                Some(ItemType::Struct(s)) => Type::Custom(s.name),
                _ => Type::try_from(&**v).unwrap(),
            };
            return Self::parse_storage_value_expression(name, key_expr, &ty, ctx);
        }

        let syn_ty = <SorobanParser as TypeParser>::parse_ty(ty, ctx)?;
        match ty {
            Type::Address => get_or_none(name, key_expr, &syn_ty),
            Type::Custom(name) => ctx
                .type_from_string(&name)
                .map(|ty| match ty {
                    ItemType::Contract(_) | ItemType::Library(_) | ItemType::Interface(_) => {
                        get_or_none(name, key_expr, &syn_ty)
                    }
                    ItemType::Enum(_) => get_or_default(name, key_expr, &syn_ty),
                    _ => get_or_revert(name, key_expr, &syn_ty),
                })
                .unwrap(),
            Type::Bool => get_or_default(name, key_expr, &syn_ty),
            Type::Uint(size) | Type::Int(size) => {
                if *size > 128 {
                    get_or_default_u256(name, key_expr, &syn_ty)
                } else {
                    get_or_default(name, key_expr, &syn_ty)
                }
            }
            Type::String => get_or_empty_string(name, key_expr, &syn_ty),
            _ => get_or_revert(name, key_expr, &syn_ty),
        }
    }

    fn parse_local_collection<T: StatementParserContext>(
        var_ident: Ident,
        keys_expr: Vec<syn::Expr>,
        value_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_storage_collection<T: StatementParserContext>(
        var_ident: Ident,
        keys_expr: Vec<syn::Expr>,
        value_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let storage = code::expr::storage_instance();
        let collection_ident = utils::to_pascal_case_ident(var_ident.to_string());
        let key: syn::Expr = parse_quote!(#collection_ident( #( #keys_expr.clone() ),*));

        match value_expr {
            Some(value) => Ok(parse_quote!(#storage.set(&#key, &#value))),
            None => Self::parse_storage_value_expression(
                var_ident.to_string().as_str(),
                Some(key),
                ty,
                ctx,
            ),
        }
    }

    fn parse_var_type(name: Ident, item_type: &Option<ItemType>) -> ParserResult<syn::Expr> {
        formatted_invalid_expr!("unknown variable {:?}", item_type)
    }

    fn caller() -> syn::Expr {
        parse_quote!(caller)
    }

    fn parse_math_op<T: StatementParserContext>(
        left: &Expression,
        right: &Expression,
        op: &MathOp,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        let right_expr = eval_in_context::<_, SorobanParser>(right, left, ctx)?;
        let left_expr = eval_in_context::<_, SorobanParser>(left, right, ctx)?;

        if op == &MathOp::Pow {
            // right is always of type u32
            let right_expr = match right {
                // ignore casting - `pow` arg must be of type u32
                Expression::Func(box Expression::Type(_), args) => {
                    var::parse_or_default::<_, Self>(args.first().unwrap(), ctx)
                }
                _ => eval_in_context::<_, SorobanParser>(right, left, ctx),
            }?;
            return Ok(parse_quote!( (#left_expr.pow(#right_expr)) ));
        }
        let ty = ctx
            .contextual_expr()
            .map(|e| eval_expression_type(e, ctx))
            .flatten()
            .unwrap_or(Type::Uint(256));
        let op: syn::BinOp = op.into();

        match ty {
            Type::Uint(size) | Type::Int(size) if size > 128 => match op {
                syn::BinOp::Add(_) => Ok(parse_quote!( (#left_expr.add(&#right_expr)) )),
                syn::BinOp::Sub(_) => Ok(parse_quote!( (#left_expr.sub(&#right_expr)) )),
                syn::BinOp::Mul(_) => Ok(parse_quote!( (#left_expr.mul(&#right_expr)) )),
                syn::BinOp::Div(_) => Ok(parse_quote!( (#left_expr.div(&#right_expr)) )),
                _ => panic!("Unknown op {:?}", op),
            },
            _ => Ok(parse_quote!(#left_expr #op #right_expr)),
        }
    }

    fn parse_update_value_expression<T: StatementParserContext>(
        current_value: syn::Expr,
        value: syn::Expr,
        op: syn::BinOp,
        ty: Type,
        ctx: &mut T,
    ) -> syn::Expr {
        match ty {
            Type::Uint(size) | Type::Int(size) if size > 128 => match op {
                syn::BinOp::Add(_) => parse_quote!(#current_value.add(&#value)),
                syn::BinOp::Sub(_) => parse_quote!(#current_value.sub(&#value)),
                syn::BinOp::Mul(_) => parse_quote!(#current_value.mul(&#value)),
                syn::BinOp::Div(_) => parse_quote!(#current_value.div(&#value)),
                _ => panic!("Unknown op {:?}", op),
            },
            _ => parse_quote!(#current_value #op #value),
        }
    }

    fn parse_ret_expr(expr: Option<syn::Expr>) -> syn::Stmt {
        code::expr::ret(expr)
    }
}

fn get_or_none(name: &str, key: Option<syn::Expr>, ty: &syn::Type) -> ParserResult<syn::Expr> {
    let key = storage_key(name, key);
    let storage_instance = code::expr::storage_instance();
    Ok(parse_quote!(#storage_instance
            .get::<_, #ty>(&#key)
            .unwrap_or(None)))
}

fn get_or_default(name: &str, key: Option<syn::Expr>, ty: &syn::Type) -> ParserResult<syn::Expr> {
    let key = storage_key(name, key);
    let storage_instance = code::expr::storage_instance();
    Ok(parse_quote!(#storage_instance
        .get::<_, #ty>(&#key)
        .unwrap_or_default()))
}

fn get_or_revert(name: &str, key: Option<syn::Expr>, ty: &syn::Type) -> ParserResult<syn::Expr> {
    let key = storage_key(name, key);
    let storage_instance = code::expr::storage_instance();
    Ok(parse_quote!(#storage_instance
        .get::<_, #ty>(&#key)
        .unwrap_or_else(|| panic!("The value is not set"))))
}

fn get_or_empty_string(
    name: &str,
    key: Option<syn::Expr>,
    ty: &syn::Type,
) -> ParserResult<syn::Expr> {
    let key = storage_key(name, key);
    let storage_instance = code::expr::storage_instance();
    let empty_string = code::expr::string_from("");
    Ok(parse_quote!(#storage_instance
        .get::<_, #ty>(&#key)
        .unwrap_or(#empty_string)))
}

fn get_or_default_u256(
    name: &str,
    key: Option<syn::Expr>,
    ty: &syn::Type,
) -> ParserResult<syn::Expr> {
    let key = storage_key(name, key);
    let storage_instance = code::expr::storage_instance();
    let zero = code::expr::default_u256();
    Ok(parse_quote!(#storage_instance
        .get::<_, #ty>(&#key)
        .unwrap_or(#zero)))
}

fn storage_key(name: &str, key: Option<syn::Expr>) -> syn::Expr {
    match key {
        Some(k) => k,
        None => {
            let key = utils::to_upper_snake_case_ident(name);
            parse_quote!(#key)
        }
    }
}
