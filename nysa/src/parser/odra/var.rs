use c3_lang_parser::c3_ast::VarDef;

use crate::{
    model::{
        ir::{NysaExpression, NysaType, NysaVar},
        ContractData,
    },
    utils::to_snake_case_ident,
};

use super::{context::Context, ty};

/// Extracts variable definitions and pareses into a vector of c3 ast [VarDef].
pub fn variables_def(data: &ContractData, ctx: &mut Context) -> Vec<VarDef> {
    data.vars().iter().map(variable_def).collect()
}

/// Transforms solidity [VariableDefinition] into a c3 ast [VarDef].
fn variable_def(v: &NysaVar) -> VarDef {
    let ident = to_snake_case_ident(&v.name);
    let ty = ty::parse_odra_ty(&v.ty);
    VarDef { ident, ty }
}

pub trait IsField {
    fn is_field(&self, ctx: &Context) -> Option<NysaType>;
}

impl<T: AsRef<str>> IsField for T {
    fn is_field(&self, ctx: &Context) -> Option<NysaType> {
        let name = self.as_ref();
        ctx.storage()
            .iter()
            .find(|v| v.name == name)
            .map(|v| v.ty.clone())
    }
}

impl IsField for &NysaExpression {
    fn is_field(&self, ctx: &Context) -> Option<NysaType> {
        let name = match self {
            NysaExpression::Variable { name } => name,
            _ => return None,
        };
        ctx.storage()
            .iter()
            .find(|f| &f.name == name)
            .map(|f| f.ty.clone())
    }
}
