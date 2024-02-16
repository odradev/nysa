use std::collections::{HashMap, HashSet};

use crate::model::ir::Package;
use crate::utils::AsStringVec;
use crate::{
    model::{
        ir::{Expression, FnImplementations, Function, InterfaceData, Stmt, Struct, Type, Var},
        ContractData, Named,
    },
    utils,
};

use super::common::StatementParserContext;

#[derive(Debug)]
pub enum ItemType {
    Contract(String),
    Library(ContractData),
    Interface(String),
    Enum(String),
    Struct(Struct),
    Event,
    Storage(Var),
    Local(Var),
}

impl ItemType {
    pub fn as_var(&self) -> Option<&Var> {
        match self {
            ItemType::Storage(v) => Some(v),
            ItemType::Local(v) => Some(v),
            _ => None,
        }
    }
}

/// Provides info about the type of an item based on its on or an expression.
///
/// Useful if we have access only to the name of eg. a variable but we need to know
/// it's type.
pub trait TypeInfo {
    fn type_from_string(&self, name: &str) -> Option<ItemType>;
    fn type_from_expression(&self, name: &Expression) -> Option<ItemType> {
        match name {
            Expression::Variable(name) => self.type_from_string(name),
            Expression::Statement(box Stmt::ReturningBlock(stmts)) => stmts
                .last()
                .map(|s| match s {
                    Stmt::Expression(e) => self.type_from_expression(e),
                    _ => None,
                })
                .flatten(),
            Expression::MemberAccess(namespace, box Expression::Variable(name)) => {
                self.type_from_string(namespace)
            }
            _ => None,
        }
    }
    fn has_enums(&self) -> bool;
    fn find_fn(&self, class: &str, name: &str) -> Option<Function>;
}

/// Provides information about the currently processing contract.
pub trait ContractInfo {
    fn current_contract(&self) -> &ContractData;
}

/// Provides information on errors defined in the whole application.
pub trait ErrorInfo {
    fn get_error<T: ToString>(&self, msg: T) -> Option<u16>;
    fn error_count(&self) -> u16;
    fn increment_error_counter(&mut self);
    fn insert_error<T: ToString>(&mut self, msg: T);
}

/// Provides info about the contract storage.
pub trait StorageInfo {
    fn storage(&self) -> Vec<Var>;
}

/// Keeps track of events emitted across the app.
pub trait EventsRegister {
    fn register_event<T: ToString>(&mut self, class: &T);
    fn emitted_events(&self) -> Vec<&String>;
}

/// Keeps track of calls to external contracts across the app.
pub trait ExternalCallsRegister {
    fn register_external_call(&mut self, class: &str);
    fn get_external_calls(&self) -> Vec<&String>;
}

/// Provides the context of the currently processed function.
pub trait FnContext {
    /// Sets the current function data.
    fn set_current_fn(&mut self, func: &FnImplementations);
    /// Drops the current function.
    fn clear_current_fn(&mut self);
    /// Returns the current function.
    fn current_fn(&self) -> &FnImplementations;
    /// Adds a new local variable in the function context.
    fn register_local_var<T: ToString>(&mut self, name: T, ty: &Type);
    /// Finds a local variable by name.
    fn get_local_var_by_name(&self, name: &str) -> Option<&Var>;
    /// Push an expression to the context stack.
    /// It adds more context to the currently processed expression.
    /// Some expressions are made of a few expressions (left and right expression
    /// in a simple case), then some expressions cannot be processed independently
    /// without knowing details of a sibling expression.
    fn push_contextual_expr(&mut self, expr: Expression) -> bool;
    /// Removes an expression from the stack.
    fn drop_contextual_expr(&mut self);
    /// Gets an expression from the top of the stack.
    fn contextual_expr(&self) -> Option<&Expression>;
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GlobalContext {
    events: Vec<String>,
    interfaces: Vec<InterfaceData>,
    libraries: Vec<ContractData>,
    enums: Vec<String>,
    errors: Vec<String>,
    classes: Vec<ContractData>,
    structs: Vec<Struct>,
    error_map: HashMap<String, u16>,
    error_count: u16,
}

impl GlobalContext {
    pub fn new(
        events: Vec<String>,
        interfaces: Vec<InterfaceData>,
        libraries: Vec<ContractData>,
        enums: Vec<String>,
        errors: Vec<String>,
        classes: Vec<ContractData>,
        structs: Vec<Struct>,
    ) -> Self {
        let error_count = errors.len() as u16;
        Self {
            events,
            interfaces,
            libraries,
            enums,
            errors,
            classes,
            structs,
            error_count,
            ..Default::default()
        }
    }
}

impl TypeInfo for GlobalContext {
    fn type_from_string(&self, name: &str) -> Option<ItemType> {
        let name = &name.to_owned();
        if let Some(l) = self.libraries.iter().find(|c| c.name() == name.to_string()) {
            return Some(ItemType::Library(l.clone()));
        }
        if self.classes.iter().any(|c| c.name() == name.to_string()) {
            return Some(ItemType::Contract(name.clone()));
        }
        if self.events.contains(name) {
            return Some(ItemType::Event);
        }
        if self.interfaces.iter().any(|c| c.name() == name.to_string()) {
            return Some(ItemType::Interface(name.clone()));
        }
        if self.enums.contains(name) {
            return Some(ItemType::Enum(name.clone()));
        }
        if let Some(s) = self.structs.iter().find(|c| c.name() == name.to_string()) {
            return Some(ItemType::Struct(s.clone()));
        }
        None
    }

    fn has_enums(&self) -> bool {
        !self.enums.is_empty()
    }

    fn find_fn(&self, class: &str, name: &str) -> Option<Function> {
        if let Some(lib) = self
            .libraries
            .iter()
            .find(|c| c.name() == class.to_string())
        {
            if let Some(f) = lib.fn_implementations().iter().find(|f| f.name == name) {
                return Some(f.implementations.first().unwrap().1.clone());
            }
        }

        if let Some(i) = self
            .interfaces
            .iter()
            .find(|c| c.name() == class.to_string())
        {
            if let Some(f) = i.fns().iter().find(|f| f.name() == name) {
                return Some(f.clone());
            }
        }

        if let Some(lib) = self.classes.iter().find(|c| c.name() == class.to_string()) {
            if let Some(f) = lib.fn_implementations().iter().find(|f| f.name == name) {
                return Some(f.implementations.first().unwrap().1.clone());
            }
        }

        None
    }
}

impl ErrorInfo for GlobalContext {
    fn error_count(&self) -> u16 {
        self.error_count
    }

    fn increment_error_counter(&mut self) {
        self.error_count += 1;
    }

    fn insert_error<T: ToString>(&mut self, msg: T) {
        self.increment_error_counter();
        self.error_map.insert(msg.to_string(), self.error_count);
    }

    fn get_error<T: ToString>(&self, msg: T) -> Option<u16> {
        self.error_map.get(&msg.to_string()).copied()
    }
}

impl Into<GlobalContext> for &Package {
    fn into(self) -> GlobalContext {
        GlobalContext::new(
            self.events().as_string_vec(),
            self.interfaces().to_vec(),
            self.libraries().to_vec(),
            self.enums().as_string_vec(),
            self.errors().as_string_vec(),
            self.contracts().to_vec(),
            self.structs().to_vec(),
        )
    }
}

#[derive(Debug)]
pub struct ContractContext<'a> {
    global: &'a mut GlobalContext,
    storage: Vec<Var>,
    external_calls: HashSet<String>,
    emitted_events: HashSet<String>,
    data: ContractData,
}

impl<'a> ContractContext<'a> {
    pub fn new(ctx: &'a mut GlobalContext, data: ContractData) -> Self {
        let storage = data
            .vars()
            .into_iter()
            .filter(|v| !v.is_immutable)
            .collect::<Vec<_>>();
        Self {
            global: ctx,
            storage,
            external_calls: Default::default(),
            emitted_events: Default::default(),
            data,
        }
    }
}

impl ContractInfo for ContractContext<'_> {
    fn current_contract(&self) -> &ContractData {
        &self.data
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

    delegate::delegate! {
        to self.global {
            fn has_enums(&self) -> bool;
            fn find_fn(&self, class: &str, name: &str) -> Option<Function>;
        }
    }
}

impl ErrorInfo for ContractContext<'_> {
    delegate::delegate! {
        to self.global {
            fn get_error<T: ToString>(&self, msg: T) -> Option<u16>;
            fn error_count(&self) -> u16;
            fn increment_error_counter(&mut self);
            fn insert_error<T: ToString>(&mut self, msg: T);
        }
    }
}

#[derive(Debug)]
pub struct LocalContext<'a> {
    contract: ContractContext<'a>,
    current_fn: Option<FnImplementations>,
    local_vars: Vec<Var>,
    contextual_expressions: Vec<Expression>,
}

impl StatementParserContext for LocalContext<'_> {}

impl<'a> LocalContext<'a> {
    pub fn new(ctx: ContractContext<'a>) -> Self {
        Self {
            contract: ctx,
            current_fn: None,
            local_vars: Default::default(),
            contextual_expressions: Default::default(),
        }
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

    delegate::delegate! {
        to self.contract {
            fn has_enums(&self) -> bool;
            fn find_fn(&self, class: &str, name: &str) -> Option<Function>;
        }
    }
}

impl FnContext for LocalContext<'_> {
    fn set_current_fn(&mut self, func: &FnImplementations) {
        self.current_fn = Some(func.clone());
    }

    fn clear_current_fn(&mut self) {
        self.current_fn = None;
        self.local_vars.clear();
    }

    fn current_fn(&self) -> &FnImplementations {
        &self
            .current_fn
            .as_ref()
            .expect("The current function should be set")
    }

    fn register_local_var<T: ToString>(&mut self, name: T, ty: &Type) {
        let var = Var {
            name: name.to_string(),
            ty: ty.to_owned(),
            initializer: None,
            is_immutable: false,
        };
        self.local_vars.push(var);
    }

    fn get_local_var_by_name(&self, name: &str) -> Option<&Var> {
        self.local_vars
            .iter()
            .find(|v| v.name == name || v.name == utils::to_snake_case(name))
    }

    fn push_contextual_expr(&mut self, expr: Expression) -> bool {
        self.contextual_expressions.push(expr);
        true
    }

    fn drop_contextual_expr(&mut self) {
        let _ = self.contextual_expressions.pop();
    }

    fn contextual_expr(&self) -> Option<&Expression> {
        self.contextual_expressions.last()
    }
}

impl ContractInfo for LocalContext<'_> {
    delegate::delegate! {
        to self.contract {
            fn current_contract(&self) -> &ContractData;
        }
    }
}

impl StorageInfo for LocalContext<'_> {
    delegate::delegate! {
        to self.contract {
            fn storage(&self) -> Vec<Var>;
        }
    }
}

impl EventsRegister for LocalContext<'_> {
    delegate::delegate! {
        to self.contract {
            fn register_event<T: ToString>(&mut self, class: &T);
            fn emitted_events(&self) -> Vec<&String>;
        }
    }
}

impl ExternalCallsRegister for LocalContext<'_> {
    delegate::delegate! {
        to self.contract {
            fn register_external_call(&mut self, class: &str);
            fn get_external_calls(&self) -> Vec<&String>;
        }
    }
}

impl ErrorInfo for LocalContext<'_> {
    delegate::delegate! {
        to self.contract {
            fn get_error<T: ToString>(&self, msg: T) -> Option<u16>;
            fn error_count(&self) -> u16;
            fn increment_error_counter(&mut self);
            fn insert_error<T: ToString>(&mut self, msg: T);
        }
    }
}

#[cfg(test)]
pub fn with_context<F: Fn(&mut LocalContext) -> ()>(f: F) {
    let data = ContractData::empty("test");
    let mut ctx = GlobalContext::default();
    let ctx = ContractContext::new(&mut ctx, data);
    let mut ctx = LocalContext::new(ctx);

    f(&mut ctx)
}

#[allow(unused_variables)]
#[cfg(test)]
pub mod test {
    use crate::{
        model::{ir::Expression, ContractData},
        parser::common::StatementParserContext,
    };

    use super::{
        ContractInfo, ErrorInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo,
        TypeInfo,
    };

    #[derive(Debug)]
    pub struct EmptyContext;

    impl StatementParserContext for EmptyContext {}

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

        fn find_fn(&self, class: &str, name: &str) -> Option<crate::model::ir::Function> {
            None
        }
    }

    impl ContractInfo for EmptyContext {
        fn current_contract(&self) -> &ContractData {
            unimplemented!()
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

        fn register_local_var<T: ToString>(&mut self, name: T, ty: &crate::model::ir::Type) {}

        fn get_local_var_by_name(&self, name: &str) -> Option<&crate::model::ir::Var> {
            todo!()
        }

        fn push_contextual_expr(&mut self, expr: Expression) -> bool {
            false
        }

        fn drop_contextual_expr(&mut self) {}

        fn contextual_expr(&self) -> Option<&Expression> {
            todo!()
        }
    }

    impl ErrorInfo for EmptyContext {
        fn get_error<T: ToString>(&self, msg: T) -> Option<u16> {
            None
        }

        fn error_count(&self) -> u16 {
            0
        }

        fn increment_error_counter(&mut self) {}

        fn insert_error<T: ToString>(&mut self, msg: T) {}
    }
}
