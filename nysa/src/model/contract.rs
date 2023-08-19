use std::collections::HashMap;

use c3_lang_linearization::{Class, C3};
use c3_lang_parser::c3_ast::ClassNameDef;
use solidity_parser::pt::{ContractDefinition, SourceUnitPart};

use crate::{
    linearization,
    utils::{self, ast, map_collection},
};

use super::ir::{NysaContract, NysaError, NysaEvent, NysaFunction, NysaVar};

type FnName = String;
type FnImpls<'a> = (FnName, Vec<(Class, NysaFunction)>);

pub struct ContractData {
    contract: NysaContract,
    events: Vec<NysaEvent>,
    errors: Vec<NysaError>,
    fn_map: HashMap<Class, Vec<NysaFunction>>,
    var_map: HashMap<Class, Vec<NysaVar>>,
    c3: C3,
}

impl TryFrom<Vec<SourceUnitPart>> for ContractData {
    type Error = &'static str;

    fn try_from(value: Vec<SourceUnitPart>) -> Result<Self, Self::Error> {
        let contracts: Vec<&ContractDefinition> = ast::extract_contracts(&value);
        let contract = contracts.last().ok_or("No contract found")?.to_owned();
        let contract = NysaContract::from(contract);

        let events = map_collection(ast::extract_events(&value));
        let errors = map_collection(ast::extract_errors(&value));

        let mut c3 = linearization::c3_linearization(&contracts);
        let mut fn_map = HashMap::new();
        let mut var_map = HashMap::new();

        let relevant_contracts = c3.all_classes_str();

        contracts
            .iter()
            .filter(|c| relevant_contracts.contains(&c.name.name))
            .for_each(|c| {
                let class = Class::from(c.name.name.as_str());
                let fns: Vec<NysaFunction> = map_collection(ast::extract_functions(c));

                for func in fns.iter() {
                    let fn_class = Class::from(func.name.as_str());
                    c3.register_fn(class.clone(), fn_class);
                }

                let vars: Vec<NysaVar> = map_collection(ast::extract_vars(c));
                for var in vars.iter() {
                    let var_class = Class::from(var.name.as_str());
                    c3.register_var(class.clone(), var_class)
                }
                fn_map.insert(class.clone(), fns);
                var_map.insert(class, vars);
            });

        Ok(Self {
            contract,
            events,
            errors,
            fn_map,
            var_map,
            c3,
        })
    }
}

impl ContractData {
    /// Extracts contract name and wraps with c3 ast abstraction.
    pub fn c3_class(&self) -> Class {
        Class::from(self.contract.name())
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

    pub fn fn_implementations(&self) -> Vec<FnImpls> {
        let mut result = vec![];
        for fn_name in self.functions_str() {
            let implementations = self
                .fn_map
                .iter()
                .filter_map(|(class, fns)| {
                    utils::func::find_by_name(class.clone(), fns.to_vec(), &fn_name)
                })
                .collect::<Vec<_>>();
            result.push((fn_name, implementations));
        }
        result
    }

    pub fn functions_str(&self) -> Vec<String> {
        self.c3.functions_str(self.contract.name())
    }

    pub fn vars(&self) -> Vec<NysaVar> {
        let mut vars = self
            .var_map
            .iter()
            .map(|(_, v)| v.clone())
            .flatten()
            .collect::<Vec<_>>();
        vars.dedup();
        vars
    }

    pub fn events(&self) -> &[NysaEvent] {
        &self.events
    }

    pub fn errors(&self) -> &[NysaError] {
        &self.errors
    }
}
