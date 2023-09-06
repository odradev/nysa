use c3_lang_parser::c3_ast::VarDef;

use crate::{
    model::{ir::NysaVar, ContractData},
    parser::context::TypeInfo,
    utils, ParserError,
};

use super::ty;

/// Extracts variable definitions and pareses into a vector of c3 ast [VarDef].
pub fn variables_def<T: TypeInfo>(
    data: &ContractData,
    t: &mut T,
) -> Result<Vec<VarDef>, ParserError> {
    data.vars().iter().map(|v| variable_def(v, t)).collect()
}

/// Transforms [NysaVar] into a c3 ast [VarDef].
fn variable_def<T: TypeInfo>(v: &NysaVar, t: &T) -> Result<VarDef, ParserError> {
    let ident = utils::to_snake_case_ident(&v.name);
    let ty = ty::parse_odra_ty(&v.ty, t)?;
    Ok(VarDef { ident, ty })
}
