use c3_lang_linearization::Fn;
use solidity_parser::pt;

pub(crate) fn find_by_name<'a>(
    contract_name: String,
    functions: Vec<&'a pt::FunctionDefinition>,
    name: &str,
) -> Option<(String, &'a pt::FunctionDefinition)> {
    let result = functions
        .iter()
        .find(|f| parse_id(f) == Fn::from(name))
        .map(|f| *f);
    result.map(|f| (contract_name, f))
}

pub(crate) fn parse_id(def: &pt::FunctionDefinition) -> Fn {
    let parse_unsafe = || -> Fn {
        def.name
            .as_ref()
            .map(|id| super::to_snake_case(&id.name))
            .expect("Invalid func name")
            .into()
    };
    match &def.ty {
        pt::FunctionTy::Constructor => "init".into(),
        pt::FunctionTy::Function => parse_unsafe(),
        pt::FunctionTy::Fallback => "__fallback".into(),
        pt::FunctionTy::Receive => "__receive".into(),
        pt::FunctionTy::Modifier => parse_unsafe(),
    }
}