use c3_lang_parser::c3_ast::VarDef;
use quote::format_ident;
use solidity_parser::pt::{ContractDefinition, ContractPart, VariableDefinition};

use crate::ty;

/// Extracts variable definitions and pareses into a vector of c3 ast [VarDef].
pub fn variables_def(contract: &ContractDefinition) -> Vec<VarDef> {
    let mut result = Vec::new();
    for maybe_var in &contract.parts {
        if let ContractPart::VariableDefinition(var_def) = maybe_var {
            result.push(variable_def(var_def));
        }
    }
    result
}

/// Transforms solidity [VariableDefinition] into a c3 ast [VarDef].
fn variable_def(v: &VariableDefinition) -> VarDef {
    let ident: proc_macro2::Ident = format_ident!("{}", v.name.name);
    let ty = ty::parse_type_from_expr(&v.ty);
    VarDef { ident, ty }
}
