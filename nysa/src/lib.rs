use c3_lang_linearization::{Class, Fn};
use c3_lang_parser::{c3_ast::{PackageDef, ClassNameDef, ClassDef, VarDef, ClassFnImpl, FnDef}};
use quote::format_ident;
use solidity_parser::pt::{self, SourceUnitPart, ContractDefinition, VariableDefinition, ContractPart, Expression, FunctionDefinition};
use syn::{parse_quote, Item, FnArg, ReturnType, Receiver, TypeTuple, punctuated::Punctuated};

pub fn parse(input: String) -> PackageDef {
    let solidity_ast = solidity_parser::parse(&input, 0).unwrap();
    let solidity_ast: &SourceUnitPart = &solidity_ast.0.0[0];
    let contract: &ContractDefinition = 
    if let SourceUnitPart::ContractDefinition(contract_def) = solidity_ast {
        contract_def
    } else {
        panic!("Not a contract def.")
    };

    let other_code = other_code();
    let class_name = class_name_def(&contract);
    let classes = vec![class_def(&contract)];
    PackageDef { other_code, class_name, classes }
}

fn class_name_def(contract: &ContractDefinition) -> ClassNameDef {
    ClassNameDef { classes: vec![class(contract)] }
}

fn class(contract: &ContractDefinition) -> Class {
    Class::from(contract.name.name.clone())
}

fn class_def(contract: &ContractDefinition) -> ClassDef {
    let variables = variables_def(contract);
    let functions = functions_def(contract);

    ClassDef { 
        struct_attrs: vec![
            parse_quote! { #[near_sdk::near_bindgen] },
            parse_quote! { #[derive(Default, BorshDeserialize, BorshSerialize)] }
        ], 
        impl_attrs: vec![
            parse_quote! { #[near_sdk::near_bindgen] },
        ],
        class: class(contract),
        path: vec![class(contract)],
        variables,
        functions
    }
}

fn functions_def(contract: &ContractDefinition) -> Vec<FnDef> {
    let class: Class = class(contract);
    contract.parts
        .iter()
        .filter_map(|part| match part {
            ContractPart::FunctionDefinition(func) => Some(func),
            _ => None
        })
        .filter(|func| func.name.is_some())
        .map(|func| function_def(*&func, class.clone()))
        .collect::<Vec<_>>()
} 

fn function_def(func: &FunctionDefinition, class: Class) -> FnDef {
    let name: Fn = func.name.to_owned().unwrap().name.into();
    let args: Vec<FnArg> = func.params
        .iter()
        .filter_map(|p| p.1.as_ref())
        .map(parameter_to_fn_arg)
        .collect();

   let attrs = func.attributes.iter()
        .filter_map(|attr| match attr {
            pt::FunctionAttribute::Mutability(m) => parse_mutability(m),
            pt::FunctionAttribute::Visibility(_) => None,
            pt::FunctionAttribute::Virtual(_) => None,
            pt::FunctionAttribute::Override(_, _) => None,
            pt::FunctionAttribute::BaseOrModifier(_, _) => None,
        })
        .collect::<Vec<_>>();

    let ret = if func.return_not_returns.is_some() {
        ReturnType::Default
    } else {
        let returns = &func.returns;
        if returns.is_empty() {
            ReturnType::Default
        } else if returns.iter().count() == 1 {
            let param = returns.first().cloned().unwrap();
            let param = param.1.unwrap();
            let ty = parse_type_from_expr(&param.ty);
            ReturnType::Type(Default::default(), Box::new(ty)) // return single value
        } else {
            let types: Punctuated<syn::Type, syn::Token![,]> = returns.iter()
                .map(|ret| parse_type_from_expr(&ret.1.as_ref().unwrap().ty) )
                .collect();
            let tuple = TypeTuple { paren_token: Default::default(), elems: types };
            ReturnType::Type(Default::default(), Box::new(syn::Type::Tuple(tuple))) // return tuple
        }
    };


    dbg!(func);


    let block: syn::Block = if let Some(v) = &func.body {
        match v {
            pt::Statement::Block { loc, unchecked, statements } => {
                let stmts = statements.iter().map(parse_statement).collect::<Vec<_>>();
                syn::Block { brace_token: Default::default(), stmts }
            },
            _ => panic!("Invalid statement - pt::Statement::Block expected")
        }
    } else {
        parse_quote! {{}}
    };

    dbg!(block.clone());

    FnDef { 
        attrs,
        name: name.clone(),
        args, 
        ret, 
        implementations: vec![
            ClassFnImpl { 
                class,
                fun: name,
                implementation: block,
            }
        ] 
    }
}

fn parse_mutability(mutability: &pt::Mutability) -> Option<syn::Attribute> {
    match mutability {
        pt::Mutability::Pure(_) => None,
        pt::Mutability::View(_) => None,
        pt::Mutability::Constant(_) => None,
        pt::Mutability::Payable(_) => Some(parse_quote!( #[payable] )),
    }
}

fn parameter_to_fn_arg(p: &pt::Parameter) -> FnArg {
    FnArg::Receiver(Receiver { attrs: vec![], reference: Some((Default::default(), None)), mutability: None, self_token: Default::default() })
}

fn variables_def(contract: &ContractDefinition) -> Vec<VarDef> {
    let mut result = Vec::new();
    for maybe_var in &contract.parts {
        if let ContractPart::VariableDefinition(var_def) = maybe_var {
            result.push(variable_def(&var_def));
        }
    }
    result
}

fn variable_def(v: &VariableDefinition) -> VarDef {
    let ident: proc_macro2::Ident = format_ident!("{}", v.name.name);
    let ty = parse_type_from_expr(&v.ty);
    VarDef { ident, ty }
}

fn parse_type_from_expr(ty: &Expression) -> syn::Type {
    match ty {
        Expression::Type(_, ty) => parse_type(ty),
        _ => panic!("Not a type.")
    }
}

fn parse_type(ty: &pt::Type) -> syn::Type {
    match ty {
        pt::Type::Mapping(_, key, value) => {
            let key = parse_type_from_expr(&key);
            let value = parse_type_from_expr(&value);
            parse_quote!{
                std::collections::HashMap<#key, #value>
            }
        },
        pt::Type::Address => parse_quote!( near_sdk::AccountId ),
        pt::Type::AddressPayable => parse_quote!( near_sdk::AccountId ),
        pt::Type::String => parse_quote!( String ),
        pt::Type::Bool => parse_quote!( bool ),
        pt::Type::Int(_) =>  parse_quote!( i16 ),
        pt::Type::Uint(_) =>  parse_quote!( u16 ),
        _ => panic!("Unsupported type.")
    }
}

fn parse_statement(stmt: &pt::Statement) -> syn::Stmt {
    match stmt {
        pt::Statement::Block { loc, unchecked, statements } => todo!(),
        pt::Statement::Assembly { loc, dialect, statements } => todo!(),
        pt::Statement::Args(_, _) => todo!(),
        pt::Statement::If(_, _, _, _) => todo!(),
        pt::Statement::While(_, _, _) => todo!(),
        pt::Statement::Expression(loc, expression) => {
            syn::Stmt::Expr(parse_expression(expression))
        },
        pt::Statement::VariableDefinition(_, declaration, expression) => {
            let name = &declaration.name.name;
            let pat: syn::Pat = parse_quote! { #name };
            syn::Stmt::Expr(syn::Expr::Let(syn::ExprLet { 
                attrs: vec![],
                let_token: Default::default(),
                pat,
                eq_token: Default::default(),
                expr: Box::new(parse_expression(expression.as_ref().unwrap()))
            }))
        },
        pt::Statement::For(_, _, _, _, _) => todo!(),
        pt::Statement::DoWhile(_, _, _) => todo!(),
        pt::Statement::Continue(_) => todo!(),
        pt::Statement::Break(_) => todo!(),
        pt::Statement::Return(_, expression) => {
            let ret = parse_expression(expression.as_ref().unwrap());
            parse_quote! {
                return #ret;
            }
        },
        pt::Statement::Revert(_, _, _) => todo!(),
        pt::Statement::Emit(_, _) => todo!(),
        pt::Statement::Try(_, _, _, _) => todo!(),
        pt::Statement::DocComment(_, _, _) => todo!(),
    }
}

fn parse_expression(expression: &pt::Expression) -> syn::Expr {
    match expression {
        Expression::PostIncrement(_, _) => todo!(),
        Expression::PostDecrement(_, _) => todo!(),
        Expression::New(_, _) => todo!(),
        Expression::ArraySubscript(_, array_expression, key_expression) => {
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
        Expression::ArraySlice(_, _, _, _) => todo!(),
        Expression::MemberAccess(_, expression, id) => {
            syn::Expr::Field(syn::ExprField { 
                attrs: vec![], 
                base: Box::new(parse_expression(&*expression)), 
                dot_token: Default::default(), 
                member: format_ident!("{}", id.name).into()
            })
        },
        Expression::FunctionCall(_, _, _) => todo!(),
        Expression::FunctionCallBlock(_, _, _) => todo!(),
        Expression::NamedFunctionCall(_, _, _) => todo!(),
        Expression::Not(_, _) => todo!(),
        Expression::Complement(_, _) => todo!(),
        Expression::Delete(_, _) => todo!(),
        Expression::PreIncrement(_, _) => todo!(),
        Expression::PreDecrement(_, _) => todo!(),
        Expression::UnaryPlus(_, _) => todo!(),
        Expression::UnaryMinus(_, _) => todo!(),
        Expression::Power(_, _, _) => todo!(),
        Expression::Multiply(_, _, _) => todo!(),
        Expression::Divide(_, _, _) => todo!(),
        Expression::Modulo(_, _, _) => todo!(),
        Expression::Add(_, _, _) => todo!(),
        Expression::Subtract(_, _, _) => todo!(),
        Expression::ShiftLeft(_, _, _) => todo!(),
        Expression::ShiftRight(_, _, _) => todo!(),
        Expression::BitwiseAnd(_, _, _) => todo!(),
        Expression::BitwiseXor(_, _, _) => todo!(),
        Expression::BitwiseOr(_, _, _) => todo!(),
        Expression::Less(_, _, _) => todo!(),
        Expression::More(_, _, _) => todo!(),
        Expression::LessEqual(_, _, _) => todo!(),
        Expression::MoreEqual(_, _, _) => todo!(),
        Expression::Equal(_, _, _) => todo!(),
        Expression::NotEqual(_, _, _) => todo!(),
        Expression::And(_, _, _) => todo!(),
        Expression::Or(_, _, _) => todo!(),
        Expression::Ternary(_, _, _, _) => todo!(),
        Expression::Assign(_, le, re) => {
            syn::Expr::Assign(syn::ExprAssign { 
                attrs: vec![], 
                left: Box::new(parse_expression(*&le)), 
                eq_token: Default::default(), 
                right: Box::new(parse_expression(*&re))
            })
        },
        Expression::AssignOr(_, _, _) => todo!(),
        Expression::AssignAnd(_, _, _) => todo!(),
        Expression::AssignXor(_, _, _) => todo!(),
        Expression::AssignShiftLeft(_, _, _) => todo!(),
        Expression::AssignShiftRight(_, _, _) => todo!(),
        Expression::AssignAdd(_, _, _) => todo!(),
        Expression::AssignSubtract(_, _, _) => todo!(),
        Expression::AssignMultiply(_, _, _) => todo!(),
        Expression::AssignDivide(_, _, _) => todo!(),
        Expression::AssignModulo(_, _, _) => todo!(),
        Expression::BoolLiteral(_, _) => todo!(),
        Expression::NumberLiteral(_, _) => todo!(),
        Expression::RationalNumberLiteral(_, _) => todo!(),
        Expression::HexNumberLiteral(_, _) => todo!(),
        Expression::StringLiteral(_) => todo!(),
        Expression::Type(_, _) => todo!(),
        Expression::HexLiteral(_) => todo!(),
        Expression::AddressLiteral(_, _) => todo!(),
        Expression::Variable(id) => {
            let name = &id.name;
            parse_quote! { #name }
        },
        Expression::List(_, _) => todo!(),
        Expression::ArrayLiteral(_, _) => todo!(),
        Expression::Unit(_, _, _) => todo!(),
        Expression::This(_) => todo!(),
    }
}

fn other_code() -> Vec<Item> {
    vec![
        parse_quote! {
            use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
        },
        parse_quote! { 
            impl BorshDeserialize for PathStack {
                fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
                    Ok(Default::default())
                }
            }
        },
        parse_quote! {
            impl BorshSerialize for PathStack {
                fn serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
                    Ok(())
                }
            }
        }
    ]
}

pub fn expected() -> PackageDef {
    PackageDef { 
        other_code: vec![
            parse_quote! {
                use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
            },
            parse_quote! { 
                impl BorshDeserialize for PathStack {
                    fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
                        Ok(Default::default())
                    }
                }
            },
            parse_quote! {
                impl BorshSerialize for PathStack {
                    fn serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
                        Ok(())
                    }
                }
            }
        ],
        class_name: ClassNameDef {
            classes: vec![Class::from("StatusMessage")],
        },
        classes: vec![ClassDef { 
            struct_attrs: vec![
                parse_quote! { #[near_sdk::near_bindgen] },
                parse_quote! { #[derive(Default, BorshDeserialize, BorshSerialize)] }
            ], 
            impl_attrs: vec![
                parse_quote! { #[near_sdk::near_bindgen] },
            ],
            class: Class::from("StatusMessage"),
            path: vec![Class::from("StatusMessage")],
            variables: vec![VarDef {
                ident: parse_quote! { records },
                ty: parse_quote! { std::collections::HashMap<near_sdk::AccountId, String> },
            }],
            functions: vec![
                FnDef {
                    attrs: vec![
                        parse_quote! { #[payable] }
                    ],
                    name: Fn::from("set_status"),
                    args: vec![parse_quote! { &mut self }, parse_quote! { message: String }],
                    ret: parse_quote! { },
                    implementations: vec![
                        ClassFnImpl {
                            class: Class::from("StatusMessage"),
                            fun: Fn::from("set_status"),
                            implementation: parse_quote! {{
                                let account_id = near_sdk::env::signer_account_id();
                                self.records.insert(account_id, message);
                            }},
                        },
                    ],
                },
                FnDef {
                    attrs: vec![],
                    name: Fn::from("get_status"),
                    args: vec![parse_quote! { &self }, parse_quote! { account_id: near_sdk::AccountId }],
                    ret: parse_quote! { -> String },
                    implementations: vec![
                        ClassFnImpl {
                            class: Class::from("StatusMessage"),
                            fun: Fn::from("get_status"),
                            implementation: parse_quote! {{
                                self.records.get(&account_id).cloned().unwrap_or_default()
                            }},
                        },
                    ],
                },
            ]
        }]
    }
}

#[cfg(test)]
mod tests {

    use c3_lang_parser::c3_ast::PackageDef;
    use pretty_assertions::assert_eq;
    use quote::ToTokens;
    use syn::parse_quote;

    use crate::{parse, expected};

    const input: &str = r#"
    contract StatusMessage {
        mapping(address => string) records;

        function set_status(string status) public payable {
            address account_id = msg.sender;
            records[account_id] = status;
        }

        function get_status(address account_id) public view returns (string memory) {
            return records[account_id];
        }
    }
    "#;

    #[test]
    fn test_parser() {
        let result: PackageDef = parse(String::from(input));
        // assert!(false);
        assert_eq!(result, expected());
    }
}
