use std::collections::HashMap;

use c3_lang_linearization::{Class, C3};
use c3_lang_parser::c3_ast::ClassNameDef;
use itertools::Itertools;
use solidity_parser::pt::ContractDefinition;

use crate::{
    c3,
    utils::{ast, map_collection},
};

use super::{
    func::{Constructor, FnImplementations, NysaFunction},
    misc::{NysaContract, NysaVar},
    Named,
};

pub struct ContractData {
    contract: NysaContract,
    all_contracts: Vec<NysaContract>,
    fn_map: HashMap<String, Vec<(Class, NysaFunction)>>,
    var_map: HashMap<Class, Vec<NysaVar>>,
    c3: C3,
}

impl TryFrom<(&Class, &Vec<&ContractDefinition>)> for ContractData {
    type Error = &'static str;

    fn try_from(value: (&Class, &Vec<&ContractDefinition>)) -> Result<Self, Self::Error> {
        let (class, contracts) = value;
        let all_contracts = contracts.iter().map(|c| NysaContract::from(*c)).collect();

        let contract = contracts
            .iter()
            .find(|c| c.name.name == class.to_string())
            .ok_or("No contract found")?
            .to_owned();
        let contract = NysaContract::from(contract);

        let mut c3 = c3::linearization(&contracts);

        let mut fn_map: HashMap<String, Vec<(Class, NysaFunction)>> = HashMap::new();
        let mut var_map = HashMap::new();

        c3.path(&contract.name().into())
            .unwrap()
            .iter()
            .rev()
            .for_each(|class| {
                let c = contracts
                    .iter()
                    .find(|c| c.name.name == class.to_string())
                    .unwrap();

                let contract = NysaContract::from(*c);

                let class = Class::from(contract.name());
                let mut fns: Vec<NysaFunction> = map_collection(ast::extract_functions(c));

                let constructor = fns
                    .iter_mut()
                    .find(|f| matches!(f, NysaFunction::Constructor(_)))
                    .map(|f| match f {
                        NysaFunction::Function(_) => None,
                        NysaFunction::Constructor(c) => Some(c),
                        NysaFunction::Modifier(_) => None,
                    })
                    .flatten();

                // There are two ways of calling a super constructor
                // ```solidity
                // contract A is B("Init X")
                // //or
                // contract A is B {
                //   constructor() B("Init X") {}
                // }
                // ```
                // So we need to pass super constructor calls from the contract level to the constructor level.
                let contract_base = contract.base_impl().to_vec();
                if let Some(c) = constructor {
                    c.extend_base(contract_base);
                } else {
                    // Each Solidity contract has a constructor even if not defined explicitly,
                    // If constructor not found in the source code, there should be created
                    // a default empty constructor.
                    let mut default_constructor = Constructor::default();
                    default_constructor.extend_base(contract_base);
                    fns.push(NysaFunction::Constructor(default_constructor));
                }

                for func in fns.iter() {
                    let fn_class = Class::from(func.name().as_str());
                    c3.register_fn(class.clone(), fn_class);

                    let fn_name = func.name();
                    let record = (class.clone(), func.clone());
                    match fn_map.get_mut(&fn_name) {
                        Some(v) => v.push(record),
                        None => {
                            fn_map.insert(fn_name.clone(), vec![record]);
                        }
                    };
                }

                let vars: Vec<NysaVar> = map_collection(ast::extract_vars(c));
                for var in vars.iter() {
                    let var_class = Class::from(var.name.as_str());
                    c3.register_var(class.clone(), var_class)
                }

                var_map.insert(class, vars);
            });

        Ok(Self {
            contract,
            all_contracts,
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

    pub fn fn_implementations(&self) -> Vec<FnImplementations> {
        let mut result = self
            .fn_map
            .iter()
            .map(|(name, implementations)| FnImplementations {
                name: name.to_owned(),
                implementations: implementations.clone(),
            })
            .collect::<Vec<_>>();
        result.sort_by_key(|f| f.name.clone());
        result
    }

    pub fn functions_str(&self) -> Vec<String> {
        self.c3.functions_str(self.contract.name().as_str())
    }

    pub fn vars(&self) -> Vec<NysaVar> {
        let class = self.c3_class().to_string();
        let c3_vars = self.c3.varialbes_str(&class);
        let mut vars = self
            .var_map
            .iter()
            .sorted()
            .map(|(_, v)| v.clone())
            .flatten()
            .filter(|v| c3_vars.contains(&v.name))
            .collect::<Vec<_>>();

        vars.dedup();
        vars
    }

    pub fn vars_to_initialize(&self) -> Vec<NysaVar> {
        let mut vars = self
            .var_map
            .iter()
            .map(|(_, v)| v.clone())
            .flatten()
            .filter(|v| v.initializer.is_some())
            .collect::<Vec<_>>();
        vars.dedup();
        vars
    }

    pub fn contract_names(&self) -> Vec<String> {
        self.all_contracts
            .iter()
            .map(|c| c.name().to_owned())
            .collect()
    }

    pub fn is_abstract(&self, class: &Class) -> bool {
        self.all_contracts
            .iter()
            .find(|c| c.name() == class.to_string())
            .map(|c| c.is_abstract())
            .unwrap_or_default()
    }
}

impl Named for ContractData {
    fn name(&self) -> String {
        self.contract.name().to_string()
    }
}
