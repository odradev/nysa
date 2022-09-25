use quote::format_ident;
use solidity_parser::pt;
use syn::parse_quote;

use super::expr::parse_expression;

pub fn parse_statement(stmt: &pt::Statement) -> syn::Stmt {
    match stmt {
        pt::Statement::Expression(loc, expression) => {
            let expr = parse_expression(expression);
            parse_quote!(#expr;)
        }
        pt::Statement::VariableDefinition(_, declaration, expression) => {
            let name = format_ident!("{}", &declaration.name.name);
            let pat: syn::Pat = parse_quote! { #name };
            let expr: syn::Expr = parse_expression(expression.as_ref().unwrap());

            parse_quote!(let #pat = #expr.clone();)
        }
        pt::Statement::Return(_, expression) => {
            let ret = parse_expression(expression.as_ref().unwrap());
            parse_quote! {
                return #ret;
            }
        }
        pt::Statement::If(_, assertion, if_body, else_body) => {
            let assertion = parse_expression(assertion);
            let if_body = parse_statement(if_body);
            let else_body = else_body.clone().unwrap();
            let else_body = parse_statement(&else_body);
            parse_quote!{
                if #assertion #if_body else #else_body
            }
        },
        pt::Statement::Block {loc: _, unchecked: _, statements } => {
            let res: Vec<syn::Stmt> = statements.iter().map(parse_statement).collect();
            parse_quote!{ { #(#res);* } }
        },
        _ => panic!("Unsupported statement {:?}", stmt)
    }
}
