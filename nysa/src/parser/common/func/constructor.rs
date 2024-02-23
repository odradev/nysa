use super::common;
use crate::{
    error::ParserResult,
    model::ir::{
        contains_sender_expr, BaseCall, Constructor, Expression, FnImplementations, Stmt, Type,
    },
    parser::common::{expr, stmt, FunctionParser, StatementParserContext},
    utils, Parser, ParserError,
};
use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassFnImpl, FnDef, PlainFnDef};
use syn::{parse_quote, Ident};

pub(super) fn def<T, P>(impls: &FnImplementations, ctx: &mut T) -> ParserResult<Vec<FnDef>>
where
    T: StatementParserContext,
    P: Parser,
{
    let impls = impls.as_constructors();

    let (_, primary_constructor) = impls
        .iter()
        .find(|(class, _)| **class == ctx.current_contract().c3_class())
        .or(impls.last())
        .ok_or(ParserError::ConstructorNotFound)?;

    impls
        .iter()
        .map(|(id, c)| {
            let attrs = <P::FnParser as FunctionParser>::constructor_attrs(c);

            let params = common::parse_params::<_, P>(&c.params, ctx)?;
            common::register_local_vars(&c.params, ctx);

            let args = <P::FnParser as FunctionParser>::parse_args(
                params,
                c.is_mutable,
                contains_sender_expr(&c.stmts),
            )?;

            let mut stmts: Vec<syn::Stmt> = vec![];
            if !ctx.current_contract().is_abstract(id) {
                stmts.extend(parse_base_calls::<_, P>(c, &impls, ctx));
            }
            stmts.extend(init_storage_fields::<_, P>(ctx)?);
            stmts.extend(common::parse_statements::<_, P>(&c.stmts, ctx));
            let name = parse_constructor_name(id, c, c == primary_constructor);

            if c == primary_constructor {
                Ok(FnDef::Plain(PlainFnDef {
                    attrs,
                    name: name.clone(),
                    args,
                    ret: common::parse_ret_type::<_, P::TypeParser>(&c.ret, ctx)?,
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
                    args,
                    ret: common::parse_ret_type::<_, P::TypeParser>(&c.ret, ctx)?,
                    implementation: ClassFnImpl {
                        class: None,
                        fun: name,
                        implementation: parse_quote!({ #(#stmts)* }),
                        visibility: parse_quote!(),
                    },
                }))
            }
        })
        .collect::<ParserResult<Vec<_>>>()
}

fn init_storage_fields<T, P>(ctx: &mut T) -> ParserResult<Vec<syn::Stmt>>
where
    T: StatementParserContext,
    P: Parser,
{
    ctx.storage()
        .iter()
        .filter(|v| v.initializer.is_some())
        .map(|v| {
            let left = match &v.ty {
                Type::Mapping(_, _) => Err(ParserError::MappingInit),
                _ => Ok(Expression::Variable(v.name.clone())),
            }?;

            let stmt = Stmt::Expression(Expression::Assign(
                Box::new(left),
                Some(Box::new(v.initializer.clone().unwrap())),
            ));
            stmt::parse_statement::<_, P>(&stmt, true, ctx)
        })
        .collect::<ParserResult<_>>()
}

fn parse_base_calls<T, P>(
    constructor: &Constructor,
    constructors: &[(&Class, &Constructor)],
    ctx: &mut T,
) -> Vec<syn::Stmt>
where
    T: StatementParserContext,
    P: Parser,
{
    let mut stmts = vec![];
    let find_base_class = |class: &Class| {
        constructor
            .base
            .iter()
            .find(|base| base.class_name == class.to_string())
    };

    constructors.iter().for_each(|(id, _)| {
        if let Some(base) = find_base_class(id) {
            let args = parse_base_args::<_, P>(base, ctx);
            let ident = parse_base_ident(base);
            let stmt = <P::FnParser as FunctionParser>::parse_base_call(ident, args);
            stmts.push(stmt);
        }
    });
    stmts
}

fn parse_base_args<T, P>(base: &BaseCall, ctx: &mut T) -> Vec<syn::Expr>
where
    T: StatementParserContext,
    P: Parser,
{
    expr::parse_many::<_, P>(&base.args, ctx).unwrap_or_default()
}

fn parse_base_ident(base: &BaseCall) -> Ident {
    let base = utils::to_snake_case(&base.class_name);
    let prefix = format!("_{}_", base);
    utils::to_prefixed_snake_case_ident(&prefix, "init")
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
