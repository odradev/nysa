use crate::parser::context::{ContractInfo, TypeInfo};
use crate::parser::soroban::code;

pub fn symbols_def<T: TypeInfo + ContractInfo>(ctx: &mut T) -> Vec<syn::ItemConst> {
    ctx.current_contract()
        .vars()
        .iter()
        .filter(|v| !v.is_immutable)
        .map(|v| code::consts::short_symbol(&v.name))
        .collect()
}
