use crate::{
    model::{
        ir::{
            Constructor, FnImplementations, NysaBaseImpl, NysaExpression, NysaStmt, NysaType,
            NysaVar,
        },
        ContractData,
    },
    parser::{
        context::Context,
        odra::{expr, stmt},
    },
    utils, ParserError,
};
use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassFnImpl, FnDef, PlainFnDef};
use syn::{parse_quote, Ident};

use super::common;

pub(super) fn def(
    impls: &FnImplementations,
    data: &ContractData,
    ctx: &mut Context,
) -> Result<Vec<FnDef>, ParserError> {
    let impls = impls.as_constructors();

    let (primary_constructor_class, primary_constructor) = impls
        .iter()
        .find(|(class, _)| **class == data.c3_class())
        .or(impls.last())
        .ok_or(ParserError::ConstructorNotFound)?;

    let stmts: Vec<syn::Stmt> = impls
        .iter()
        .map(|(_, c)| common::parse_statements(&c.stmts, ctx))
        .flatten()
        .collect();

    impls
        .iter()
        .map(|(id, c)| {
            let mut attrs = vec![];
            if c.is_payable {
                attrs.push(parse_quote!(#[odra(payable)]));
            }

            let mut stmts: Vec<syn::Stmt> = vec![];
            if !data.is_abstract(id) {
                stmts.extend(parse_base_calls(c, &impls, ctx));
            }
            stmts.extend(init_storage_fields(ctx)?);
            stmts.extend(common::parse_statements(&c.stmts, ctx));
            let name = parse_constructor_name(id, c, c == primary_constructor);

            if c == primary_constructor {
                attrs.push(parse_quote!(#[odra(init)]));

                Ok(FnDef::Plain(PlainFnDef {
                    attrs,
                    name: name.clone(),
                    args: common::args(&c.params, c.is_mutable)?,
                    ret: common::parse_ret_type(&c.ret)?,
                    implementation: ClassFnImpl {
                        class: None,
                        fun: name,
                        implementation: parse_quote!({ #(#stmts)* }),
                        visibility: parse_quote!(pub),
                    },
                }))
            } else {
                Ok(FnDef::Plain(PlainFnDef {
                    attrs,
                    name: name.clone(),
                    args: common::args(&c.params, c.is_mutable)?,
                    ret: common::parse_ret_type(&c.ret)?,
                    implementation: ClassFnImpl {
                        class: None,
                        fun: name,
                        implementation: parse_quote!({ #(#stmts)* }),
                        visibility: parse_quote!(),
                    },
                }))
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

fn init_storage_fields(ctx: &mut Context) -> Result<Vec<syn::Stmt>, ParserError> {
    ctx.storage()
        .iter()
        .filter(|v| v.initializer.is_some())
        .map(
            |NysaVar {
                 name,
                 ty,
                 initializer,
             }| {
                let init_expr = initializer.clone().unwrap();
                let left = match ty {
                    NysaType::Mapping(k, v) => Err(ParserError::MappingInit),
                    _ => Ok(NysaExpression::Variable { name: name.clone() }),
                }?;

                let stmt = NysaStmt::Expression {
                    expr: NysaExpression::Assign {
                        left: Box::new(left),
                        right: Box::new(initializer.clone().unwrap()),
                    },
                };
                stmt::parse_statement(&stmt, ctx)
            },
        )
        .collect::<Result<_, _>>()
}

fn parse_base_calls(
    constructor: &Constructor,
    constructors: &[(&Class, &Constructor)],
    ctx: &mut Context,
) -> Vec<syn::Stmt> {
    let mut stmts = vec![];
    let find_base_class = |class: &Class| {
        constructor
            .base
            .iter()
            .find(|base| base.class_name == class.to_string())
    };

    constructors.iter().for_each(|(id, i)| {
        if let Some(base) = find_base_class(id) {
            let args = parse_base_args(base, ctx);
            let ident = parse_base_ident(base);
            stmts.push(parse_quote!(self.#ident( #(#args),* );));
        }
    });
    stmts
}

fn parse_base_args(base: &NysaBaseImpl, ctx: &mut Context) -> Vec<syn::Expr> {
    expr::parse_many(&base.args, ctx).unwrap_or(vec![])
}

fn parse_base_ident(base: &NysaBaseImpl) -> Ident {
    let base = utils::to_snake_case(&base.class_name);
    let prefix = format!("_{}_", base);
    let ident = utils::to_prefixed_snake_case_ident(&prefix, "init");
    ident
}

fn parse_constructor_name(class: &Class, constructor: &Constructor, is_primary: bool) -> Class {
    if is_primary {
        constructor.name.as_str().into()
    } else {
        let name = format!(
            "_{}_{}",
            utils::to_snake_case(class.to_string().as_str()),
            constructor.name
        );
        name.as_str().into()
    }
}
