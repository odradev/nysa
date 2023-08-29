use c3_lang_parser::c3_ast::VarDef;

use crate::{
    model::{
        ir::{NysaExpression, NysaType, NysaVar},
        ContractData,
    },
    utils, ParserError,
};

use super::{context::Context, ty};

/// Extracts variable definitions and pareses into a vector of c3 ast [VarDef].
pub fn variables_def(data: &ContractData, ctx: &mut Context) -> Result<Vec<VarDef>, ParserError> {
    data.vars().iter().map(variable_def).collect()
}

/// Transforms [NysaVar] into a c3 ast [VarDef].
fn variable_def(v: &NysaVar) -> Result<VarDef, ParserError> {
    let ident = utils::to_snake_case_ident(&v.name);
    let ty = ty::parse_odra_ty(&v.ty)?;
    Ok(VarDef { ident, ty })
}

pub trait AsVariable {
    fn as_var(&self, ctx: &Context) -> Result<NysaType, ParserError>;
}

impl<T: AsRef<str>> AsVariable for T {
    fn as_var(&self, ctx: &Context) -> Result<NysaType, ParserError> {
        let name = self.as_ref();
        ctx.storage()
            .iter()
            .find(|v| v.name == name)
            .map(|v| v.ty.clone())
            .ok_or(ParserError::NotStateVariable)
    }
}

impl AsVariable for &NysaExpression {
    fn as_var(&self, ctx: &Context) -> Result<NysaType, ParserError> {
        let name = match self {
            NysaExpression::Variable { name } => name,
            _ => return Err(ParserError::NotStateVariable),
        };
        ctx.storage()
            .iter()
            .find(|f| &f.name == name)
            .map(|f| f.ty.clone())
            .ok_or(ParserError::NotStateVariable)
    }
}
