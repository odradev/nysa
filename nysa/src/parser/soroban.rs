use super::common::StatementParserContext;
use super::{context::GlobalContext, Parser};
use crate::error::ParserResult;
use crate::model::ir::{Expression, Type};
use crate::parser::common::{
    ContractErrorParser, ContractReferenceParser, EventEmitParser, ExpressionParser, NumberParser,
    StringParser, TypeParser,
};
use crate::parser::context::{ContractContext, ContractInfo, LocalContext};
use crate::{model::ir::Package, utils, ParserError};
use c3_lang_parser::c3_ast::{ClassDef, PackageDef};
use proc_macro2::TokenStream;

pub(crate) mod code;
mod func;
mod symbol;
#[cfg(test)]
mod test;
mod ty;
mod var;

/// Implementation of [Parser]. Generates code compatible with the Soroban Framework.
pub struct SorobanParser;

impl Parser for SorobanParser {
    fn parse(package: Package) -> Result<TokenStream, ParserError> {
        // register all metadata in the global context.
        let mut ctx: GlobalContext = (&package).into();

        let packages = parse_packages(&package, &mut ctx)?;

        let contracts = packages
            .iter()
            .map(|def| {
                let name = def.classes.first().as_ref().unwrap().class.to_string();
                let mod_name = utils::to_snake_case_ident(&name);
                quote::quote! {
                    pub mod #mod_name {
                        #def
                    }
                }
            })
            .collect::<TokenStream>();

        Ok(quote::quote! {
            pub mod errors {
            }

            pub mod events {
            }

            pub mod enums {
            }

            pub mod structs {
            }

            #contracts
        })
    }

    type EventEmitParser = Self;
    type ContractReferenceParser = Self;
    type ContractErrorParser = Self;
    type ExpressionParser = Self;
}

impl EventEmitParser for SorobanParser {
    fn parse_emit_stmt(
        event_ident: proc_macro2::Ident,
        args: Vec<syn::Expr>,
    ) -> ParserResult<syn::Stmt> {
        todo!()
    }
}

impl ContractReferenceParser for SorobanParser {
    fn parse_contract_ref(variable_name: &str, contract_name: &str) -> syn::Stmt {
        todo!()
    }

    fn parse_contract_ref_expr(contract_name: &str, address: syn::Expr) -> syn::Expr {
        todo!()
    }
}

impl ContractErrorParser for SorobanParser {
    fn revert_with_str<T: StatementParserContext>(
        condition: Option<&Expression>,
        message: &str,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn revert_with_err(error_name: &str) -> syn::Expr {
        todo!()
    }

    fn revert<T: StatementParserContext>(
        condition: Option<&Expression>,
        error: &Expression,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        todo!()
    }
}

impl NumberParser for SorobanParser {
    fn parse_typed_number<T: StatementParserContext>(
        values: &[u64],
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_generic_number(expr: &Expression) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn unsigned_one() -> syn::Expr {
        todo!()
    }
}

impl StringParser for SorobanParser {
    fn parse_string(string: &str) -> ParserResult<syn::Expr> {
        todo!()
    }
}

impl TypeParser for SorobanParser {
    fn parse_ty<T: StatementParserContext>(ty: &Type, ctx: &mut T) -> ParserResult<syn::Type> {
        todo!()
    }

    fn parse_fixed_bytes(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_serialize(args: Vec<syn::Expr>) -> ParserResult<syn::Expr> {
        todo!()
    }
}

impl ExpressionParser for SorobanParser {
    fn parse_set_var_expression(
        var_expression: syn::Expr,
        value_expr: syn::Expr,
        item_type: Option<super::context::ItemType>,
    ) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_read_values_expression<
        F: quote::ToTokens,
        T: super::context::StorageInfo + super::context::TypeInfo,
    >(
        field: F,
        key_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> syn::Expr {
        todo!()
    }

    fn parse_local_collection<T: StatementParserContext>(
        var_ident: proc_macro2::Ident,
        keys_expr: Vec<syn::Expr>,
        value_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        todo!()
    }

    fn parse_storage_collection<T: StatementParserContext>(
        var_ident: proc_macro2::Ident,
        keys_expr: Vec<syn::Expr>,
        value_expr: Option<syn::Expr>,
        ty: &Type,
        ctx: &mut T,
    ) -> ParserResult<syn::Expr> {
        todo!()
    }
}

fn parse_packages(package: &Package, ctx: &mut GlobalContext) -> ParserResult<Vec<PackageDef>> {
    package
        .contracts()
        .iter()
        .map(|data| {
            let mut ctx = LocalContext::new(ContractContext::new(ctx, data.clone()));
            let class_name = data.c3_class_name_def();
            // let contract_def = contract_def(&mut ctx)?;
            let storage_items = symbol::symbols_def(&mut ctx)
                .into_iter()
                .map(From::from)
                .collect();
            Ok(PackageDef {
                no_std: true,
                attrs: vec![],
                other_code: storage_items,
                class_name,
                classes: vec![contract_def(&mut ctx)?],
            })
        })
        .collect::<ParserResult<_>>()
}

fn contract_def(ctx: &mut LocalContext) -> ParserResult<ClassDef> {
    // let constants = var::const_def(ctx)?;
    // let functions = func::functions_def(ctx)?;

    Ok(ClassDef {
        struct_attrs: vec![code::attr::contract()],
        impl_attrs: vec![code::attr::contractimpl()],
        class: ctx.current_contract().c3_class(),
        path: ctx.current_contract().c3_path(),
        variables: vec![],
        functions: vec![],
        other_items: vec![],
    })
}
