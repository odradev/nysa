use std::collections::HashMap;

use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::ClassNameDef;
use solidity_parser::pt::ContractDefinition;

use crate::{
    c3,
    utils::{ast, map_collection},
};

use super::{
    func::{Constructor, FnImplementations, Function},
    misc::{ContractMetadata, LibUsing, Var},
    Named,
};

/// A complete smart contract representation.
/// In contrast to solidity representation, it has a flat structure.
///
/// Contains all information about inherited contracts, functions, state variables, libraries used.
#[derive(Debug, Clone)]
pub struct ContractData {
    contract: ContractMetadata,
    all_contracts: Vec<ContractMetadata>,
    functions: Vec<FnImplementations>,
    vars: Vec<Var>,
    libs: Vec<LibUsing>,
    c3_path: Vec<Class>,
}

impl TryFrom<(&Class, &Vec<&ContractDefinition>)> for ContractData {
    type Error = &'static str;

    fn try_from(value: (&Class, &Vec<&ContractDefinition>)) -> Result<Self, Self::Error> {
        let (class, contracts) = value;

        // extract the main contract definition
        let contract: ContractMetadata = extract_contract(class, contracts)
            .ok_or("No contract found")?
            .to_owned()
            .into();

        let c3 = c3::linearization(&contracts);

        let mut fn_map: HashMap<String, Vec<(Class, Function)>> = HashMap::new();
        // let mut var_map = HashMap::new();
        let mut libs = Vec::<LibUsing>::new();
        let mut vars = vec![];
        // Iterate over all the classes from the inheritance graph and pull out variables, functions and libs
        c3.path(&contract.name().into())
            .unwrap()
            .into_iter()
            .rev()
            .for_each(|class| {
                let def = extract_contract(&class, contracts).unwrap();

                libs.extend(map_collection(ast::extract_using(def)));
                let mut fns: Vec<Function> = map_collection(ast::extract_functions(def));

                let constructor = fns
                    .iter_mut()
                    .filter_map(|f| match f {
                        Function::Constructor(c) => Some(c),
                        _ => None,
                    })
                    .last();

                // There are two ways of calling a super constructor
                // ```solidity
                // contract A is B("Init X")
                // //or
                // contract A is B {
                //   constructor() B("Init X") {}
                // }
                // ```
                // So we need to pass super constructor calls from the contract level to the constructor level.
                let meta = ContractMetadata::from(def);
                let contract_base = meta.base_impl();
                if let Some(c) = constructor {
                    c.extend_base(contract_base);
                } else {
                    // Each Solidity contract has a constructor even if not defined explicitly,
                    // If constructor not found in the source code, there should be created
                    // a default empty constructor.
                    let mut default_constructor = Constructor::default();
                    default_constructor.extend_base(contract_base);
                    fns.push(Function::Constructor(default_constructor));
                }

                for func in fns {
                    let fn_name = func.name();
                    let record = (class.clone(), func);
                    match fn_map.get_mut(&fn_name) {
                        Some(v) => v.push(record),
                        None => {
                            fn_map.insert(fn_name, vec![record]);
                        }
                    };
                }
                vars.extend(map_collection(ast::extract_vars(def)));
            });
        let all_contracts = contracts
            .iter()
            .map(|c| ContractMetadata::from(*c))
            .collect();

        let mut functions = fn_map
            .iter()
            .map(|(name, impls)| FnImplementations::new(name, impls))
            .collect::<Vec<_>>();
        functions.sort_by_key(|f| f.name.clone());

        Ok(Self {
            contract,
            all_contracts,
            functions,
            vars,
            libs,
            c3_path: c3.path(class).expect("Invalid contract path"),
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
        self.c3_path.clone()
    }

    /// Returns contract functions (from the main contract and inherited).
    pub fn fn_implementations(&self) -> Vec<FnImplementations> {
        self.functions.clone()
    }

    /// Checks if a function with a given name exists in the contract.
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.iter().find(|f| &f.name == name).is_some()
    }

    /// Returns state variables (from the main contract and inherited).
    pub fn vars(&self) -> Vec<Var> {
        self.vars.clone()
    }

    /// Returns if the contract of a given `class` is abstract.
    pub fn is_abstract(&self, class: &Class) -> bool {
        self.all_contracts
            .iter()
            .find(|c| c.name() == class.to_string())
            .map(|c| c.is_abstract())
            .unwrap_or_default()
    }

    /// Returns if the contract is a library.
    pub fn is_library(&self) -> bool {
        self.contract.is_library()
    }

    /// Returns libraries used by the contract.
    pub fn libs(&self) -> &[LibUsing] {
        self.libs.as_ref()
    }
}

impl Named for ContractData {
    fn name(&self) -> String {
        self.contract.name().to_string()
    }
}

fn extract_contract<'a>(
    class: &Class,
    contracts: &'a [&ContractDefinition],
) -> Option<&'a ContractDefinition> {
    contracts
        .iter()
        .find(|c| c.name.name == class.to_string())
        .copied()
}

#[cfg(test)]
impl ContractData {
    pub fn empty<R: AsRef<str>>(name: R) -> Self {
        Self {
            contract: ContractMetadata::new(name.as_ref().to_string(), vec![], false, false),
            all_contracts: vec![ContractMetadata::new(
                name.as_ref().to_string(),
                vec![],
                false,
                false,
            )],
            functions: Default::default(),
            vars: Default::default(),
            libs: Default::default(),
            c3_path: vec![],
        }
    }

    pub fn with_storage<R: AsRef<str>>(name: R, vars: Vec<Var>) -> Self {
        Self {
            contract: ContractMetadata::new(name.as_ref().to_string(), vec![], false, false),
            all_contracts: vec![ContractMetadata::new(
                name.as_ref().to_string(),
                vec![],
                false,
                false,
            )],
            functions: Default::default(),
            libs: Default::default(),
            vars,
            c3_path: vec![],
        }
    }
}
