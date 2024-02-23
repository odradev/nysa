use crate::{
    error::ParserResult,
    model::{ir::Package, Named},
    parser::context::TypeInfo,
    utils, Parser,
};

use super::{func, ExtContractParser};

pub(crate) fn ext_contracts_def<T: TypeInfo, P: Parser>(
    package: &Package,
    ctx: &T,
) -> ParserResult<Vec<syn::ItemMod>> {
    let interfaces = package.interfaces();

    interfaces
        .iter()
        .map(|i| {
            let ident = utils::to_ident(i.name());
            let fns: Vec<syn::TraitItem> = i
                .fns()
                .iter()
                .map(|f| func::interface::def::<_, P>(f, ctx))
                .collect::<ParserResult<Vec<_>>>()?;

            let mod_ident = utils::to_snake_case_ident(i.name());
            <P::ElementsParser as ExtContractParser>::parse_ext_contract(mod_ident, ident, fns)
        })
        .collect()
}
