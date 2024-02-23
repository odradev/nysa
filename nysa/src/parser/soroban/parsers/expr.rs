use proc_macro2::Ident;
use syn::parse_quote;

use crate::{
    error::ParserResult,
    formatted_invalid_expr,
    model::ir::Type,
    parser::{
        common::{
            expr::math::eval_in_context, ExpressionParser, StatementParserContext, TypeParser,
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
        let key: syn::Expr = parse_quote!(#collection_ident( #( #keys_expr ),*));

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
        left: &crate::model::ir::Expression,
        right: &crate::model::ir::Expression,
        op: &crate::model::ir::MathOp,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        // u256 must use the `add`, `sub`, `mul`, `div`, `rem` functions
        let op: syn::BinOp = op.into();
        let left_expr = eval_in_context::<_, SorobanParser>(left, right, ctx)?;
        let right_expr = eval_in_context::<_, SorobanParser>(right, left, ctx)?;
        Ok(parse_quote!( (#left_expr #op #right_expr) ))
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
