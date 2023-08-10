use c3_lang_parser::c3_ast::VarDef;

use crate::{
    model::{ir::NysaVar, ContractData},
    utils::to_snake_case_ident,
};

use super::ty;

/// Extracts variable definitions and pareses into a vector of c3 ast [VarDef].
pub fn variables_def(data: &ContractData) -> Vec<VarDef> {
    data.vars().iter().map(variable_def).collect()
}

/// Transforms solidity [VariableDefinition] into a c3 ast [VarDef].
fn variable_def(v: &NysaVar) -> VarDef {
    let ident = to_snake_case_ident(&v.name);
    let ty = ty::parse_type_from_expr(&v.ty);
    VarDef { ident, ty }
}

pub trait IsField {
    fn is_field(&self, fields: &[NysaVar]) -> bool;
}

impl<T: AsRef<str>> IsField for T {
    fn is_field(&self, fields: &[NysaVar]) -> bool {
        let fields = fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        fields.contains(&self.as_ref().to_string())
    }
}
