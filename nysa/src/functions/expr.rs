use quote::format_ident;
use solidity_parser::pt;
use syn::parse_quote;

pub fn parse_expression(expression: &pt::Expression) -> syn::Expr {
    match expression {
        pt::Expression::PostIncrement(_, _) => todo!(),
        pt::Expression::PostDecrement(_, _) => todo!(),
        pt::Expression::New(_, _) => todo!(),
        pt::Expression::ArraySubscript(_, array_expression, key_expression) => {
            let array = parse_expression(*&array_expression);

            if let Some(exp) = key_expression {
                let key = parse_expression(*&exp);
                parse_quote! {
                    #array[#key]
                }
            } else {
                panic!("Unspecified key");
            }
        },
        pt::Expression::ArraySlice(_, _, _, _) => todo!(),
        pt::Expression::MemberAccess(_, expression, id) => {
            // dbg!(id);
            // dbg!(expression);
            match expression.as_ref() {
                pt::Expression::Variable(var) => {
                    if &var.name == "msg" && &id.name == "sender" {
                        parse_quote!(near_sdk::env::signer_account_id())
                    } else {
                        panic!("Unknown variable");
                    }
                },
                _ => {
                    let base_expr: syn::Expr = parse_expression(&*expression);
                    let member: syn::Member = format_ident!("{}", id.name).into();
                    parse_quote!(#base_expr.#member)
                }
            }
        },
        pt::Expression::FunctionCall(_, _, _) => todo!(),
        pt::Expression::FunctionCallBlock(_, _, _) => todo!(),
        pt::Expression::NamedFunctionCall(_, _, _) => todo!(),
        pt::Expression::Not(_, _) => todo!(),
        pt::Expression::Complement(_, _) => todo!(),
        pt::Expression::Delete(_, _) => todo!(),
        pt::Expression::PreIncrement(_, _) => todo!(),
        pt::Expression::PreDecrement(_, _) => todo!(),
        pt::Expression::UnaryPlus(_, _) => todo!(),
        pt::Expression::UnaryMinus(_, _) => todo!(),
        pt::Expression::Power(_, _, _) => todo!(),
        pt::Expression::Multiply(_, _, _) => todo!(),
        pt::Expression::Divide(_, _, _) => todo!(),
        pt::Expression::Modulo(_, _, _) => todo!(),
        pt::Expression::Add(_, _, _) => todo!(),
        pt::Expression::Subtract(_, _, _) => todo!(),
        pt::Expression::ShiftLeft(_, _, _) => todo!(),
        pt::Expression::ShiftRight(_, _, _) => todo!(),
        pt::Expression::BitwiseAnd(_, _, _) => todo!(),
        pt::Expression::BitwiseXor(_, _, _) => todo!(),
        pt::Expression::BitwiseOr(_, _, _) => todo!(),
        pt::Expression::Less(_, _, _) => todo!(),
        pt::Expression::More(_, _, _) => todo!(),
        pt::Expression::LessEqual(_, _, _) => todo!(),
        pt::Expression::MoreEqual(_, _, _) => todo!(),
        pt::Expression::Equal(_, _, _) => todo!(),
        pt::Expression::NotEqual(_, _, _) => todo!(),
        pt::Expression::And(_, _, _) => todo!(),
        pt::Expression::Or(_, _, _) => todo!(),
        pt::Expression::Ternary(_, _, _, _) => todo!(),
        pt::Expression::Assign(_, le, re) => {
            syn::Expr::Assign(syn::ExprAssign { 
                attrs: vec![], 
                left: Box::new(parse_expression(*&le)), 
                eq_token: Default::default(), 
                right: Box::new(parse_expression(*&re))
            })
        },
        pt::Expression::AssignOr(_, _, _) => todo!(),
        pt::Expression::AssignAnd(_, _, _) => todo!(),
        pt::Expression::AssignXor(_, _, _) => todo!(),
        pt::Expression::AssignShiftLeft(_, _, _) => todo!(),
        pt::Expression::AssignShiftRight(_, _, _) => todo!(),
        pt::Expression::AssignAdd(_, _, _) => todo!(),
        pt::Expression::AssignSubtract(_, _, _) => todo!(),
        pt::Expression::AssignMultiply(_, _, _) => todo!(),
        pt::Expression::AssignDivide(_, _, _) => todo!(),
        pt::Expression::AssignModulo(_, _, _) => todo!(),
        pt::Expression::BoolLiteral(_, _) => todo!(),
        pt::Expression::NumberLiteral(_, _) => todo!(),
        pt::Expression::RationalNumberLiteral(_, _) => todo!(),
        pt::Expression::HexNumberLiteral(_, _) => todo!(),
        pt::Expression::StringLiteral(_) => todo!(),
        pt::Expression::Type(_, _) => todo!(),
        pt::Expression::HexLiteral(_) => todo!(),
        pt::Expression::AddressLiteral(_, _) => todo!(),
        pt::Expression::Variable(id) => {
            let name = &id.name;
            parse_quote! { #name }
        },
        pt::Expression::List(_, _) => todo!(),
        pt::Expression::ArrayLiteral(_, _) => todo!(),
        pt::Expression::Unit(_, _, _) => todo!(),
        pt::Expression::This(_) => todo!(),
    }
}
