use c3_lang_linearization::{Class, Fn, C3};
use c3_lang_parser::c3_ast::ClassNameDef;
use solidity_parser::pt::{ContractDefinition, FunctionDefinition, VariableDefinition};

use crate::{linearization::parse_func_id, utils};

type FnImpls<'a> = (String, Vec<(String, &'a FunctionDefinition)>);
pub struct ContractData<'a> {
    contract: &'a ContractDefinition,
    base_contracts: Vec<&'a ContractDefinition>,
    c3: C3,
}

impl<'a> ContractData<'a> {
    pub fn new(
        contract: &'a ContractDefinition,
        base_contracts: Vec<&'a ContractDefinition>,
        c3: C3,
    ) -> Self {
        Self {
            contract,
            base_contracts,
            c3,
        }
    }

    /// Extracts contract name and wraps with c3 ast abstraction.
    pub fn c3_class(&self) -> Class {
        Class::from(self.contract.name.name.as_str())
    }

    // Extracts contract name and wraps with c3 ast abstraction.
    ///
    /// May contain one or more class name
    pub fn c3_class_name_def(&self) -> ClassNameDef {
        ClassNameDef {
            classes: self.c3_path(),
        }
    }

    /// Extracts contract name with inherited contracts and wraps with c3 ast abstraction.
    pub fn c3_path(&self) -> Vec<Class> {
        let contract_id = self.c3_class();
        self.c3.path(&contract_id).expect("Invalid contract path")
    }

    pub fn c3_fn_names(&self) -> Vec<String> {
        let contract_name_str = self.contract.name.name.as_str();
        self.c3.functions_str(contract_name_str)
    }

    pub fn c3_fn_implementations(&self) -> Vec<FnImpls> {
        let all_functions = self.all_functions();
        let mut result = vec![];
        for fn_name in self.c3_fn_names() {
            let implementations = self
                .all_functions()
                .into_iter()
                .filter_map(|(contract_name, functions)| {
                    find_fn(contract_name, functions, &fn_name)
                })
                .collect::<Vec<_>>();
            result.push((fn_name, implementations));
        }
        result
    }

    fn all_functions(&self) -> Vec<(String, Vec<&FunctionDefinition>)> {
        self.base_contracts
            .iter()
            .map(|contract| {
                (
                    contract.name.name.clone(),
                    utils::extract_functions(contract),
                )
            })
            .collect::<Vec<_>>()
    }

    pub fn c3_vars(&self) -> Vec<&VariableDefinition> {
        let mut vars = self
            .base_contracts
            .iter()
            .map(|contract| utils::extract_vars(contract))
            .flatten()
            .collect::<Vec<_>>();
        vars.dedup();
        vars
    }
}

fn find_fn<'a>(
    contract_name: String,
    functions: Vec<&'a FunctionDefinition>,
    name: &str,
) -> Option<(String, &'a FunctionDefinition)> {
    let result = functions
        .iter()
        .find(|f| parse_func_id(f) == Fn::from(name))
        .map(|f| *f);
    result.map(|f| (contract_name, f))
}
