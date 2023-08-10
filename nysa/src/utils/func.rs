use c3_lang_linearization::Class;

use crate::model::ir::NysaFunction;

pub(crate) fn find_by_name(
    contract_name: Class,
    functions: Vec<NysaFunction>,
    name: &str,
) -> Option<(Class, NysaFunction)> {
    let result = functions.iter().find(|f| f.name == name).map(|f| f.clone());
    result.map(|f| (contract_name, f))
}
