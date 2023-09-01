use std::collections::HashSet;

use crate::model::ir::{FnImplementations, NysaExpression, NysaVar};

#[allow(dead_code)]
pub enum ItemType {
    Contract,
    Interface,
    Enum(String),
    Event,
    Storage,
    Unknown,
}

#[derive(Debug, Default)]
pub struct Context<'a> {
    current_fn: Option<FnImplementations>,
    storage: &'a [NysaVar],
    events: Vec<String>,
    interfaces: Vec<String>,
    enums: Vec<String>,
    errors: Vec<String>,
    classes: Vec<String>,
    external_calls: HashSet<String>,
    emitted_events: HashSet<String>,
}

impl<'a> Context<'a> {
    pub fn set_storage(&mut self, storage: &'a [NysaVar]) {
        self.storage = storage
    }

    pub fn storage(&self) -> &'a [NysaVar] {
        self.storage
    }

    pub fn set_current_fn(&mut self, func: &FnImplementations) {
        self.current_fn = Some(func.clone());
    }

    pub fn clear_current_fn(&mut self) {
        self.current_fn = None;
    }

    #[allow(dead_code)]
    pub fn current_fn(&self) -> &FnImplementations {
        &self
            .current_fn
            .as_ref()
            .expect("The current function should be set")
    }

    pub fn set_classes(&mut self, classes: Vec<String>) {
        if !self.classes.is_empty() {
            panic!("Classes can be set once")
        }

        self.classes = classes;
    }

    pub fn class(&self, name: &NysaExpression) -> Option<String> {
        match name {
            NysaExpression::Variable { name } => {
                self.classes.contains(name).then(|| name.to_owned())
            }
            _ => None,
        }
    }

    pub fn is_class(&self, name: &String) -> bool {
        self.classes.contains(name)
    }

    pub fn item_type(&self, name: &String) -> ItemType {
        if self.classes.contains(name) {
            return ItemType::Contract;
        }
        if self.events.contains(name) {
            return ItemType::Event;
        }
        if self.interfaces.contains(name) {
            return ItemType::Interface;
        }
        if self.enums.contains(name) {
            return ItemType::Enum(name.clone());
        }

        return ItemType::Unknown;
    }

    pub fn item_type2(&self, name: &NysaExpression) -> ItemType {
        match name {
            NysaExpression::Variable { name } => self.item_type(name),
            _ => ItemType::Unknown,
        }
    }

    pub fn register_external_call(&mut self, class: &str) {
        self.external_calls.insert(class.to_owned());
    }

    pub fn get_external_calls(&self) -> Vec<&String> {
        self.external_calls.iter().collect::<Vec<_>>()
    }

    pub fn register_event(&mut self, class: &str) {
        self.emitted_events.insert(class.to_owned());
    }

    pub fn emitted_events(&self) -> Vec<&String> {
        self.emitted_events.iter().collect::<Vec<_>>()
    }

    pub fn set_events(&mut self, events: Vec<String>) {
        self.events = events;
    }

    pub fn set_interfaces(&mut self, interfaces: Vec<String>) {
        self.interfaces = interfaces;
    }

    pub fn set_enums(&mut self, enums: Vec<String>) {
        self.enums = enums;
    }

    pub fn set_errors(&mut self, errors: Vec<String>) {
        self.errors = errors;
    }

    pub fn has_enums(&self) -> bool {
        !self.enums.is_empty()
    }
}
