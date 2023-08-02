use c3_lang_parser::c3_ast::VarDef;
use solidity_parser::pt::{Identifier, VariableDefinition};

use crate::{model::ContractData, ty, utils::to_snake_case_ident};

/// Extracts variable definitions and pareses into a vector of c3 ast [VarDef].
pub fn variables_def(data: &ContractData) -> Vec<VarDef> {
    data.c3_vars().iter().map(|var| variable_def(var)).collect()
}

/// Transforms solidity [VariableDefinition] into a c3 ast [VarDef].
fn variable_def(v: &VariableDefinition) -> VarDef {
    let ident = to_snake_case_ident(&v.name.name);
    let ty = ty::parse_type_from_expr(&v.ty);
    VarDef { ident, ty }
}

pub trait IsField {
    fn is_field(&self, fields: &[&VariableDefinition]) -> bool;
}

impl IsField for &Identifier {
    fn is_field(&self, fields: &[&VariableDefinition]) -> bool {
        let fields = fields
            .iter()
            .map(|f| f.name.name.clone())
            .collect::<Vec<_>>();
        let result = fields.contains(&self.name);
        result
    }
}
