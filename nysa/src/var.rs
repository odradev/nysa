use c3_lang_parser::c3_ast::VarDef;
use solidity_parser::pt::{ContractDefinition, ContractPart, VariableDefinition, Identifier};

use crate::{ty, utils::to_snake_case_ident};

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
    let ident = to_snake_case_ident(&v.name.name);
    let ty = ty::parse_type_from_expr(&v.ty);
    VarDef { ident, ty }
}

pub trait IsField {
    fn is_field(&self, fields: &[VarDef]) -> bool;
}

// impl <T: AsRef<str>> IsField for T {
//     fn is_field(&self, fields: &[VarDef]) -> bool {
//         let fields = fields.iter().map(|f| f.ident.to_string().as_str()).collect::<Vec<_>>();
//         fields.contains(&self.as_ref())
//     }
// }

impl IsField for &Identifier {
    fn is_field(&self, fields: &[VarDef]) -> bool {
        let fields = fields.iter().map(|f| f.ident.to_string()).collect::<Vec<_>>();
        fields.contains(&self.name)
    }
}