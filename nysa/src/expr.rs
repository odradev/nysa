use quote::format_ident;
use solidity_parser::pt;
use syn::parse_quote;

pub fn parse_expression(expression: &pt::Expression) -> syn::Expr {
    match expression {
        pt::Expression::ArraySubscript(_, array_expression, key_expression) => {
            let array = parse_expression(array_expression);

            if let Some(exp) = key_expression {
                let key = parse_expression(exp);
                parse_quote! {
                    self.#array.get(&#key).cloned().unwrap_or_default()
                }
            } else {
                panic!("Unspecified key");
            }
        }
        pt::Expression::MemberAccess(_, expression, id) => match expression.as_ref() {
            pt::Expression::Variable(var) => {
                if &var.name == "msg" && &id.name == "sender" {
                    parse_quote!(near_sdk::env::signer_account_id())
                } else {
                    panic!("Unknown variable");
                }
            }
            _ => {
                let base_expr: syn::Expr = parse_expression(&*expression);
                let member: syn::Member = format_ident!("{}", id.name).into();
                parse_quote!(#base_expr.#member)
            }
        },
        pt::Expression::Assign(_, le, re) => {
            let le: &pt::Expression = le;
            let re: &pt::Expression = re;
            if let pt::Expression::ArraySubscript(_, array_expr, key_expr) = le {
                let array = parse_expression(array_expr);
                let key = parse_expression(&key_expr.clone().unwrap());
                let value = parse_expression(re);
                parse_quote! {
                    self.#array.insert(#key, #value)
                }
            } else {
                panic!("Unsupported expr assign");
            }
        }
        pt::Expression::Variable(id) => {
            let name = format_ident!("{}", &id.name);
            parse_quote! { #name }
        }
        pt::Expression::FunctionCall(_, name, args) => {
            let name = parse_expression(name);
            let args: Vec<syn::Expr> = args.iter().map(parse_expression).collect();
            parse_quote! { self.#name(#(#args),*) }
        }
        pt::Expression::LessEqual(_, l, r) => {
            let l = parse_expression(l);
            let r = parse_expression(r);
            parse_quote! { #l <= #r }
        }
        pt::Expression::NumberLiteral(_, num) => {
            let (sign, digs) = num.to_u32_digits();
            let num = digs[0];
            parse_quote!(#num)
        }
        pt::Expression::Add(_, l, r) => {
            let l = parse_expression(l);
            let r = parse_expression(r);
            parse_quote! { #l + #r }
        }
        pt::Expression::Subtract(_, l, r) => {
            let l = parse_expression(l);
            let r = parse_expression(r);
            parse_quote! { #l - #r }
        }
        _ => panic!("Unsupported expression {:?}", expression),
    }
}
