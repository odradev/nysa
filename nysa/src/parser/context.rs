use std::collections::HashSet;

use crate::model::ir::{Expression, FnImplementations, Type, Var};

#[derive(Debug)]
pub enum ItemType {
    Contract(String),
    Interface(String),
    Enum(String),
    Event,
    Storage(Var),
    Local(Var),
}

pub trait TypeInfo {
    fn type_from_string(&self, name: &str) -> Option<ItemType>;
    fn type_from_expression(&self, name: &Expression) -> Option<ItemType> {
        match name {
            Expression::Variable(name) => self.type_from_string(name),
            _ => None,
        }
    }
    fn has_enums(&self) -> bool;
}

pub trait ContractInfo {
    fn as_contract_name(&self, name: &Expression) -> Option<String>;
    fn is_class(&self, name: &str) -> bool;
}

pub trait StorageInfo {
    fn storage(&self) -> Vec<Var>;
}

pub trait EventsRegister {
    fn register_event<T: ToString>(&mut self, class: &T);
    fn emitted_events(&self) -> Vec<&String>;
}

pub trait ExternalCallsRegister {
    fn register_external_call(&mut self, class: &str);
    fn get_external_calls(&self) -> Vec<&String>;
}

pub trait FnContext {
    fn set_current_fn(&mut self, func: &FnImplementations);
    fn clear_current_fn(&mut self);
    fn current_fn(&self) -> &FnImplementations;
    fn register_local_var<T: ToString>(&mut self, name: &T, ty: &Type);
    fn get_local_var_by_name(&self, name: &str) -> Option<&Var>;
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GlobalContext {
    events: Vec<String>,
    interfaces: Vec<String>,
    enums: Vec<String>,
    errors: Vec<String>,
    classes: Vec<String>,
}

impl GlobalContext {
    pub fn new(
        events: Vec<String>,
        interfaces: Vec<String>,
        enums: Vec<String>,
        errors: Vec<String>,
        classes: Vec<String>,
    ) -> Self {
        Self {
            events,
            interfaces,
            enums,
            errors,
            classes,
        }
    }

    pub fn as_contract_name(&self, name: &Expression) -> Option<String> {
        match self.type_from_expression(name) {
            Some(ItemType::Contract(c)) => Some(c),
            Some(ItemType::Interface(i)) => Some(i),
            _ => None,
        }
    }

    pub fn is_class(&self, name: &String) -> bool {
        self.classes.contains(name)
    }
}

impl TypeInfo for GlobalContext {
    fn type_from_string(&self, name: &str) -> Option<ItemType> {
        let name = &name.to_owned();
        if self.classes.contains(name) {
            return Some(ItemType::Contract(name.clone()));
        }
        if self.events.contains(name) {
            return Some(ItemType::Event);
        }
        if self.interfaces.contains(name) {
            return Some(ItemType::Interface(name.clone()));
        }
        if self.enums.contains(name) {
            return Some(ItemType::Enum(name.clone()));
        }
        None
    }

    fn has_enums(&self) -> bool {
        !self.enums.is_empty()
    }
}

#[derive(Debug)]
pub struct ContractContext<'a> {
    global: &'a GlobalContext,
    storage: &'a [Var],
    external_calls: HashSet<String>,
    emitted_events: HashSet<String>,
}

impl<'a> ContractContext<'a> {
    pub fn new(ctx: &'a GlobalContext, storage: &'a [Var]) -> Self {
        Self {
            global: ctx,
            storage,
            external_calls: Default::default(),
            emitted_events: Default::default(),
        }
    }
}

impl ExternalCallsRegister for ContractContext<'_> {
    fn register_external_call(&mut self, class: &str) {
        self.external_calls.insert(class.to_owned());
    }

    fn get_external_calls(&self) -> Vec<&String> {
        self.external_calls.iter().collect::<Vec<_>>()
    }
}

impl EventsRegister for ContractContext<'_> {
    fn register_event<T: ToString>(&mut self, class: &T) {
        self.emitted_events.insert(class.to_string());
    }

    fn emitted_events(&self) -> Vec<&String> {
        self.emitted_events.iter().collect::<Vec<_>>()
    }
}

impl StorageInfo for ContractContext<'_> {
    fn storage(&self) -> Vec<Var> {
        self.storage.to_vec()
    }
}

impl ContractInfo for ContractContext<'_> {
    fn as_contract_name(&self, name: &Expression) -> Option<String> {
        self.global.as_contract_name(name)
    }

    fn is_class(&self, name: &str) -> bool {
        self.global.is_class(&name.to_string())
    }
}

impl TypeInfo for ContractContext<'_> {
    fn type_from_string(&self, name: &str) -> Option<ItemType> {
        let super_result = self.global.type_from_string(name);
        if super_result.is_some() {
            return super_result;
        }
        if let Some(v) = self.storage.iter().find(|v| &v.name == name) {
            return Some(ItemType::Storage(v.clone()));
        }
        None
    }

    fn has_enums(&self) -> bool {
        self.global.has_enums()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LocalContext<'a> {
    contract: ContractContext<'a>,
    current_fn: Option<FnImplementations>,
    local_vars: Vec<Var>,
}

impl<'a> LocalContext<'a> {
    pub fn new(ctx: ContractContext<'a>) -> Self {
        Self {
            contract: ctx,
            current_fn: None,
            local_vars: Default::default(),
        }
    }
}

impl ExternalCallsRegister for LocalContext<'_> {
    fn register_external_call(&mut self, class: &str) {
        self.contract.register_external_call(class);
    }

    fn get_external_calls(&self) -> Vec<&String> {
        self.contract.get_external_calls()
    }
}

impl EventsRegister for LocalContext<'_> {
    fn register_event<T: ToString>(&mut self, class: &T) {
        self.contract.register_event(class);
    }

    fn emitted_events(&self) -> Vec<&String> {
        self.contract.emitted_events()
    }
}

impl StorageInfo for LocalContext<'_> {
    fn storage(&self) -> Vec<Var> {
        self.contract.storage()
    }
}

impl ContractInfo for LocalContext<'_> {
    fn as_contract_name(&self, name: &Expression) -> Option<String> {
        self.contract.as_contract_name(name)
    }

    fn is_class(&self, name: &str) -> bool {
        self.contract.is_class(name)
    }
}

impl TypeInfo for LocalContext<'_> {
    fn type_from_string(&self, name: &str) -> Option<ItemType> {
        let super_result = self.contract.type_from_string(name);
        if super_result.is_some() {
            return super_result;
        }
        if let Some(v) = self.get_local_var_by_name(name) {
            return Some(ItemType::Local(v.clone()));
        }
        None
    }

    fn has_enums(&self) -> bool {
        self.contract.has_enums()
    }
}

impl FnContext for LocalContext<'_> {
    fn set_current_fn(&mut self, func: &FnImplementations) {
        self.current_fn = Some(func.clone());
    }

    fn clear_current_fn(&mut self) {
        self.current_fn = None;
    }

    fn current_fn(&self) -> &FnImplementations {
        &self
            .current_fn
            .as_ref()
            .expect("The current function should be set")
    }

    fn register_local_var<T: ToString>(&mut self, name: &T, ty: &Type) {
        let var = Var {
            name: name.to_string(),
            ty: ty.to_owned(),
            initializer: None,
        };
        self.local_vars.push(var);
    }

    fn get_local_var_by_name(&self, name: &str) -> Option<&Var> {
        self.local_vars.iter().find(|v| v.name == name)
    }
}

#[cfg(test)]
pub mod test {
    use crate::model::ir::Expression;

    use super::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    };

    pub struct EmptyContext;

    impl StorageInfo for EmptyContext {
        fn storage(&self) -> Vec<crate::model::ir::Var> {
            vec![]
        }
    }

    impl TypeInfo for EmptyContext {
        fn type_from_string(&self, name: &str) -> Option<crate::parser::context::ItemType> {
            None
        }

        fn has_enums(&self) -> bool {
            false
        }
    }

    impl ContractInfo for EmptyContext {
        fn as_contract_name(&self, name: &Expression) -> Option<String> {
            None
        }

        fn is_class(&self, name: &str) -> bool {
            false
        }
    }

    impl EventsRegister for EmptyContext {
        fn register_event<T: ToString>(&mut self, class: &T) {}

        fn emitted_events(&self) -> Vec<&String> {
            vec![]
        }
    }

    impl ExternalCallsRegister for EmptyContext {
        fn register_external_call(&mut self, class: &str) {}

        fn get_external_calls(&self) -> Vec<&String> {
            vec![]
        }
    }

    impl FnContext for EmptyContext {
        fn set_current_fn(&mut self, func: &crate::model::ir::FnImplementations) {}

        fn clear_current_fn(&mut self) {}

        fn current_fn(&self) -> &crate::model::ir::FnImplementations {
            todo!()
        }

        fn register_local_var<T: ToString>(&mut self, name: &T, ty: &crate::model::ir::Type) {}

        fn get_local_var_by_name(&self, name: &str) -> Option<&crate::model::ir::Var> {
            todo!()
        }
    }
}
