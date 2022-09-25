use quote::format_ident;
use solidity_parser::pt;
use syn::parse_quote;

use super::expr::parse_expression;

pub fn parse_statement(stmt: &pt::Statement) -> syn::Stmt {
    match stmt {
        pt::Statement::Block {
            loc,
            unchecked,
            statements,
        } => todo!(),
        pt::Statement::Assembly {
            loc,
            dialect,
            statements,
        } => todo!(),
        pt::Statement::Args(_, _) => todo!(),
        pt::Statement::If(_, _, _, _) => todo!(),
        pt::Statement::While(_, _, _) => todo!(),
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
        pt::Statement::For(_, _, _, _, _) => todo!(),
        pt::Statement::DoWhile(_, _, _) => todo!(),
        pt::Statement::Continue(_) => todo!(),
        pt::Statement::Break(_) => todo!(),
        pt::Statement::Return(_, expression) => {
            let ret = parse_expression(expression.as_ref().unwrap());
            parse_quote! {
                return #ret;
            }
        }
        pt::Statement::Revert(_, _, _) => todo!(),
        pt::Statement::Emit(_, _) => todo!(),
        pt::Statement::Try(_, _, _, _) => todo!(),
        pt::Statement::DocComment(_, _, _) => todo!(),
    }
}
