use std::collections::HashSet;

use crate::model::ir::{FnImplementations, NysaExpression, NysaType, NysaVar};

#[derive(Debug)]
pub enum ItemType {
    Contract(String),
    Interface(String),
    Enum(String),
    Event,
    Storage(NysaVar),
    Local(NysaVar),
}

pub trait TypeInfo {
    fn type_from_string(&self, name: &str) -> Option<ItemType>;
    fn type_from_expression(&self, name: &NysaExpression) -> Option<ItemType> {
        match name {
            NysaExpression::Variable { name } => self.type_from_string(name),
            _ => None,
        }
    }
    fn has_enums(&self) -> bool;
}

pub trait ContractInfo {
    fn as_contract_name(&self, name: &NysaExpression) -> Option<String>;
    fn is_class(&self, name: &str) -> bool;
}

pub trait StorageInfo {
    fn storage(&self) -> Vec<NysaVar>;
}

pub trait EventsRegister {
    fn register_event(&mut self, class: &str);
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
    fn register_local_var(&mut self, name: &str, ty: &NysaType);
    fn get_local_var_by_name(&self, name: &str) -> Option<&NysaVar>;
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

    pub fn as_contract_name(&self, name: &NysaExpression) -> Option<String> {
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
    storage: &'a [NysaVar],
    external_calls: HashSet<String>,
    emitted_events: HashSet<String>,
}

impl<'a> ContractContext<'a> {
    pub fn new(ctx: &'a GlobalContext, storage: &'a [NysaVar]) -> Self {
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
    fn register_event(&mut self, class: &str) {
        self.emitted_events.insert(class.to_owned());
    }

    fn emitted_events(&self) -> Vec<&String> {
        self.emitted_events.iter().collect::<Vec<_>>()
    }
}

impl StorageInfo for ContractContext<'_> {
    fn storage(&self) -> Vec<NysaVar> {
        self.storage.to_vec()
    }
}

impl ContractInfo for ContractContext<'_> {
    fn as_contract_name(&self, name: &NysaExpression) -> Option<String> {
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
    local_vars: Vec<NysaVar>,
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
    fn register_event(&mut self, class: &str) {
        self.contract.register_event(class);
    }

    fn emitted_events(&self) -> Vec<&String> {
        self.contract.emitted_events()
    }
}

impl StorageInfo for LocalContext<'_> {
    fn storage(&self) -> Vec<NysaVar> {
        self.contract.storage()
    }
}

impl ContractInfo for LocalContext<'_> {
    fn as_contract_name(&self, name: &NysaExpression) -> Option<String> {
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

    fn register_local_var(&mut self, name: &str, ty: &NysaType) {
        let var = NysaVar {
            name: name.to_owned(),
            ty: ty.to_owned(),
            initializer: None,
        };
        self.local_vars.push(var);
    }

    fn get_local_var_by_name(&self, name: &str) -> Option<&NysaVar> {
        self.local_vars.iter().find(|v| v.name == name)
    }
}
