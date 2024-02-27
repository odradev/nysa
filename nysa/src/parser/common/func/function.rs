use c3_lang_parser::c3_ast::{ClassFnImpl, ComplexFnDef, FnDef, PlainFnDef};
use proc_macro2::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, Token};

use crate::{
    error::ParserResult,
    model::{
        ir::{BaseCall, FnImplementations, Func, Function, Stmt, Type},
        Named,
    },
    parser::{
        common::{expr, ExpressionParser, FunctionParser, StatementParserContext},
        syn_utils,
    },
    utils, Parser,
};

use super::common;

/// Transforms [Var] into a c3 ast [FnDef].
pub(super) fn def<T, P>(impls: &FnImplementations, ctx: &mut T) -> ParserResult<FnDef>
where
    T: StatementParserContext,
    P: Parser,
{
    let definitions = impls.as_functions();

    let (_, top_lvl_func) = definitions
        .iter()
        .find(|(class, _)| **class == ctx.current_contract().c3_class())
        .or(definitions.last())
        .expect("At least one implementation expected")
        .clone();

    let attrs = <P::FnParser as FunctionParser>::attrs(top_lvl_func);

    let base_args = common::parse_params::<_, P>(&top_lvl_func.params, ctx)?;
    common::register_local_vars(&top_lvl_func.params, ctx);

    // let all_stmts = all_stmts(Function::Function(top_lvl_func.clone()), ctx);

    let args =
        <P::FnParser as FunctionParser>::parse_args(base_args, top_lvl_func.is_mutable, true)?;

    let implementations = definitions
        .iter()
        .map(|(class, def)| ClassFnImpl {
            class: Some(class.to_owned().clone()),
            fun: def.name.clone().into(),
            implementation: parse_body::<_, P>(def, ctx),
            visibility: common::parse_visibility(&def.vis),
        })
        .collect();

    Ok(FnDef::Complex(ComplexFnDef {
        attrs,
        name: top_lvl_func.name.as_str().into(),
        args,
        ret: common::parse_ret_type::<_, P::TypeParser>(&top_lvl_func.ret, ctx)?,
        implementations,
    }))
}

/// Transforms [Var] into a c3 ast [FnDef].
pub(super) fn library_def<T, P>(impls: &FnImplementations, ctx: &mut T) -> ParserResult<FnDef>
where
    T: StatementParserContext,
    P: Parser,
{
    let functions = impls.as_functions();
    // only one impl expected
    let (id, func) = functions.first().unwrap();

    let attrs = <P::FnParser as FunctionParser>::attrs(func);

    let args = common::parse_params::<_, P>(&func.params, ctx)?;
    common::register_local_vars(&func.params, ctx);
    // let args = <P::FnParser as FunctionParser>::parse_args(params, func.is_mutable)?;

    let implementation = ClassFnImpl {
        class: None,
        fun: func.name.clone().into(),
        implementation: parse_body::<_, P>(func, ctx),
        visibility: common::parse_visibility(&func.vis),
    };

    Ok(FnDef::Plain(PlainFnDef {
        attrs,
        name: func.name.as_str().into(),
        args,
        ret: common::parse_ret_type::<_, P::TypeParser>(&func.ret, ctx)?,
        implementation,
    }))
}

fn parse_body<T, P>(def: &Func, ctx: &mut T) -> syn::Block
where
    T: StatementParserContext,
    P: Parser,
{
    def.ret
        .iter()
        .filter_map(|(name, ty)| match name {
            Some(n) => Some((n, ty)),
            None => None,
        })
        .filter_map(|(name, e)| match Type::try_from(e) {
            Ok(ty) => Some((name, ty)),
            Err(_) => None,
        })
        .for_each(|(name, ty)| {
            ctx.register_local_var(name, &ty);
        });

    let ret_names = def
        .ret
        .iter()
        .filter_map(|(name, ty)| match name {
            Some(n) => Some(n),
            None => None,
        })
        .map(utils::to_snake_case_ident)
        .map(syn_utils::default_value)
        .collect::<Vec<_>>();

    let ret = def
        .ret
        .iter()
        .filter_map(|(name, _)| match name {
            Some(n) => Some(n),
            None => None,
        })
        .map(|n| utils::to_snake_case_ident(n))
        .map(|i| quote::quote!(#i))
        .collect::<Punctuated<TokenStream, Token![,]>>();
    let ret: Option<syn::Expr> = match ret.len() {
        0 => None,
        _ => Some(parse_quote!( (#ret) )),
    };

    // parse solidity function body
    let stmts: Vec<syn::Stmt> = common::parse_statements::<_, P>(&def.stmts, ctx);

    let ret = if !def.stmts.iter().any(|s| matches!(s, Stmt::Return(_))) {
        Some(<P::ExpressionParser as ExpressionParser>::parse_ret_expr(
            ret,
        ))
    } else {
        None
    };

    let ext = common::parse_external_contract_statements::<_, P::ContractReferenceParser>(
        &def.params,
        ctx,
    );

    // handle constructor of modifiers calls;
    // Eg `constructor(string memory _name) Named(_name) {}`
    // Eg `function mint(address _to, uint256 _amount) public onlyOwner {}`
    let before_stmts = def
        .modifiers
        .iter()
        .filter_map(|BaseCall { class_name, args }| {
            let args = expr::parse_many::<_, P>(args, ctx).unwrap_or_default();
            if ctx
                .current_contract()
                .has_function(&utils::to_snake_case(class_name))
            {
                // modifier call
                let ident = utils::to_prefixed_snake_case_ident("modifier_before_", class_name);

                Some(<P::FnParser as FunctionParser>::parse_modifier_call(
                    ident, args,
                ))
            } else {
                // super constructor call but handled already
                None
            }
        })
        .collect::<Vec<syn::Stmt>>();

    let after_stmts = def
        .modifiers
        .iter()
        .rev()
        .filter_map(|BaseCall { class_name, args }| {
            let args = expr::parse_many::<_, P>(&args, ctx).unwrap_or_default();
            if ctx
                .current_contract()
                .has_function(&utils::to_snake_case(class_name))
            {
                // modifier call
                let ident = utils::to_prefixed_snake_case_ident("modifier_after_", class_name);
                Some(<P::FnParser as FunctionParser>::parse_modifier_call(
                    ident, args,
                ))
            } else {
                // super constructor call but handled already
                None
            }
        })
        .collect::<Vec<syn::Stmt>>();
    parse_quote!({
        #(#ret_names)*
        #(#before_stmts)*
        #(#ext)*
        #(#stmts)*
        #(#after_stmts)*
        #ret
    })
}

#[allow(dead_code)]
fn all_stmts<T: StatementParserContext>(f: Function, ctx: &T) -> Vec<Stmt> {
    let path = ctx
        .current_contract()
        .c3_path()
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>();
    let calls = ctx.call_list(path.clone(), &f.name());

    let all_stmts = path
        .iter()
        .map(|class| {
            calls
                .iter()
                .filter_map(|name| ctx.find_fn(class, name))
                .map(|f| f.stmts())
                .collect::<Vec<_>>()
        })
        .flatten()
        .flatten()
        .chain(f.stmts())
        .collect::<Vec<_>>();
    all_stmts
}
