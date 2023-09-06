use quote::ToTokens;
use syn::parse_quote;

use super::parse_statement;
use crate::{
    model::ir::{NysaExpression, NysaStmt},
    parser::context::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    },
};

struct EmptyContext;

impl StorageInfo for EmptyContext {
    fn storage(&self) -> Vec<crate::model::ir::NysaVar> {
        vec![]
    }
}

impl TypeInfo for EmptyContext {
    fn type_from_string(&self, name: &str) -> Option<crate::parser::context::ItemType> {
        None
    }

    fn has_enums(&self) -> bool {
        false
    }
}

impl ContractInfo for EmptyContext {
    fn as_contract_name(&self, name: &NysaExpression) -> Option<String> {
        None
    }

    fn is_class(&self, name: &str) -> bool {
        false
    }
}

impl EventsRegister for EmptyContext {
    fn register_event(&mut self, class: &str) {}

    fn emitted_events(&self) -> Vec<&String> {
        vec![]
    }
}

impl ExternalCallsRegister for EmptyContext {
    fn register_external_call(&mut self, class: &str) {}

    fn get_external_calls(&self) -> Vec<&String> {
        vec![]
    }
}

impl FnContext for EmptyContext {
    fn set_current_fn(&mut self, func: &crate::model::ir::FnImplementations) {}

    fn clear_current_fn(&mut self) {}

    fn current_fn(&self) -> &crate::model::ir::FnImplementations {
        todo!()
    }

    fn register_local_var(&mut self, name: &str, ty: &crate::model::ir::NysaType) {
        todo!()
    }

    fn get_local_var_by_name(&self, name: &str) -> Option<&crate::model::ir::NysaVar> {
        todo!()
    }
}

#[test]
fn revert_with_no_msg() {
    let stmt = NysaStmt::Revert { msg: None };
    let result = parse_statement(&stmt, &mut EmptyContext).unwrap();
    let expected: syn::Stmt =
        parse_quote!(odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "")););

    assert(result, expected);
}

#[test]
fn revert_with_msg() {
    let error_msg = "An error occurred";
    let stmt = NysaStmt::Revert {
        msg: Some(NysaExpression::StringLiteral(error_msg.to_string())),
    };
    let result = parse_statement(&stmt, &mut EmptyContext).unwrap();
    let expected: syn::Stmt = parse_quote!(
        odra::contract_env::revert(odra::types::ExecutionError::new(1u16, "An error occurred"));
    );

    assert(result, expected)
}

#[test]
fn revert_with_error() {
    let error_msg = "MyError";
    let stmt = NysaStmt::RevertWithError {
        error: error_msg.to_string(),
    };
    let result = parse_statement(&stmt, &mut EmptyContext).unwrap();
    let expected: syn::Stmt = parse_quote!(odra::contract_env::revert(Error::MyError););

    assert(result, expected)
}

#[test]
fn invalid_revert_stmt() {
    let error_msg = "An error occurred";
    let stmt = NysaStmt::Revert {
        msg: Some(NysaExpression::Placeholder),
    };
    let result = parse_statement(&stmt, &mut EmptyContext);

    assert!(result.is_err());
}

fn assert<L, R>(left: L, right: R)
where
    L: ToTokens,
    R: ToTokens,
{
    assert_eq!(
        left.into_token_stream().to_string(),
        right.into_token_stream().to_string()
    )
}
