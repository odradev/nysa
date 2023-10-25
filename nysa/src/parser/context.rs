use std::collections::HashSet;

use crate::{
    model::{
        ir::{Expression, FnImplementations, Function, InterfaceData, Stmt, Type, Var},
        ContractData, Named,
    },
    utils,
};

#[derive(Debug)]
pub enum ItemType {
    Contract(String),
    Library(String),
    Interface(String),
    Enum(String),
    Struct(String),
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
            _ => None,
        }
    }
    fn has_enums(&self) -> bool;
    fn find_fn(&self, class: &str, name: &str) -> Option<Function>;
}

pub trait ContractInfo {
    fn current_contract(&self) -> &ContractData;
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
    fn push_expected_type(&mut self, ty: Option<Type>) -> bool;
    fn drop_expected_type(&mut self);
    fn expected_type(&self) -> Option<&Type>;
}

pub trait ExpressionContext {
    fn current_expr(&self) -> &Expression;
    fn set_expr(&mut self, expr: &Expression);
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
    structs: Vec<String>,
}

impl GlobalContext {
    pub fn new(
        events: Vec<String>,
        interfaces: Vec<InterfaceData>,
        libraries: Vec<ContractData>,
        enums: Vec<String>,
        errors: Vec<String>,
        classes: Vec<ContractData>,
        structs: Vec<String>,
    ) -> Self {
        Self {
            events,
            interfaces,
            libraries,
            enums,
            errors,
            classes,
            structs,
        }
    }

    pub fn as_contract_name(&self, name: &Expression) -> Option<String> {
        match self.type_from_expression(name) {
            Some(ItemType::Contract(c)) => Some(c),
            Some(ItemType::Interface(i)) => Some(i),
            _ => None,
        }
    }

    pub fn is_class(&self, name: &str) -> bool {
        self.classes.iter().any(|c| c.name() == name.to_string())
    }
}

impl TypeInfo for GlobalContext {
    fn type_from_string(&self, name: &str) -> Option<ItemType> {
        let name = &name.to_owned();
        if self.libraries.iter().any(|c| c.name() == name.to_string()) {
            return Some(ItemType::Library(name.clone()));
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
        if self.structs.contains(name) {
            return Some(ItemType::Struct(name.clone()));
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

#[derive(Debug)]
pub struct ContractContext<'a> {
    global: &'a GlobalContext,
    storage: Vec<Var>,
    external_calls: HashSet<String>,
    emitted_events: HashSet<String>,
    data: ContractData,
}

impl<'a> ContractContext<'a> {
    pub fn new(ctx: &'a GlobalContext, data: ContractData) -> Self {
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
    fn as_contract_name(&self, name: &Expression) -> Option<String> {
        self.global.as_contract_name(name)
    }
    fn is_class(&self, name: &str) -> bool {
        self.global.is_class(name)
    }
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

    fn has_enums(&self) -> bool {
        self.global.has_enums()
    }

    fn find_fn(&self, class: &str, name: &str) -> Option<Function> {
        self.global.find_fn(class, name)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LocalContext<'a> {
    contract: ContractContext<'a>,
    current_fn: Option<FnImplementations>,
    local_vars: Vec<Var>,
    expected_types: Vec<Type>,
}

impl<'a> LocalContext<'a> {
    pub fn new(ctx: ContractContext<'a>) -> Self {
        Self {
            contract: ctx,
            current_fn: None,
            local_vars: Default::default(),
            expected_types: Default::default(),
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

    fn has_enums(&self) -> bool {
        self.contract.has_enums()
    }

    fn find_fn(&self, class: &str, name: &str) -> Option<Function> {
        self.contract.find_fn(class, name)
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

    fn register_local_var<T: ToString>(&mut self, name: &T, ty: &Type) {
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

    fn push_expected_type(&mut self, ty: Option<Type>) -> bool {
        let result = ty.is_some();
        match ty {
            Some(Type::Array(ty)) => self.expected_types.push(*ty),
            Some(ty) => self.expected_types.push(ty),
            _ => {}
        };
        result
    }

    fn drop_expected_type(&mut self) {
        let _ = self.expected_types.pop();
    }

    fn expected_type(&self) -> Option<&Type> {
        self.expected_types.last()
    }
}

macro_rules! delegate_external_calls_register {
    ( <$lifetime:tt, $g:tt>, $ty:ty, $to:ident) => {
        impl<$lifetime, $g: ExternalCallsRegister> ExternalCallsRegister for $ty {
            delegate_external_calls_register!($to);
        }
    };
    ($ty:ty, $to:ident) => {
        impl ExternalCallsRegister for $ty {
            delegate_external_calls_register!($to);
        }
    };
    ($to:ident) => {
        fn register_external_call(&mut self, class: &str) {
            self.$to.register_external_call(class);
        }

        fn get_external_calls(&self) -> Vec<&String> {
            self.$to.get_external_calls()
        }
    };
}

macro_rules! delegate_events_register {
    ( <$lifetime:tt, $g:tt>, $ty:ty, $to:ident) => {
        impl<$lifetime, $g: EventsRegister> EventsRegister for $ty {
            delegate_events_register!($to);
        }
    };
    ($ty:ty, $to:ident) => {
        impl EventsRegister for $ty {
            delegate_events_register!($to);
        }
    };
    ($to:ident) => {
        fn register_event<T: ToString>(&mut self, class: &T) {
            self.$to.register_event(class);
        }

        fn emitted_events(&self) -> Vec<&String> {
            self.$to.emitted_events()
        }
    };
}

macro_rules! delegate_storage_info {
    ( <$lifetime:tt, $g:tt>, $ty:ty, $to:ident) => {
        impl<$lifetime, $g: StorageInfo> StorageInfo for $ty {
            delegate_storage_info!($to);
        }
    };
    ( $ty:ty, $to:ident) => {
        impl StorageInfo for $ty {
            delegate_storage_info!($to);
        }
    };
    ($to:ident) => {
        fn storage(&self) -> Vec<Var> {
            self.$to.storage()
        }
    };
}

macro_rules! delegate_contract_info {
    ( <$lifetime:tt, $g:tt>, $ty:ty, $to:ident) => {
        impl<$lifetime, $g: ContractInfo> ContractInfo for $ty {
            fn as_contract_name(&self, name: &Expression) -> Option<String> {
                self.$to.as_contract_name(name)
            }

            fn is_class(&self, name: &str) -> bool {
                self.$to.is_class(name)
            }
        }
    };
    ($ty:ty, $to:ident) => {
        impl ContractInfo for $ty {
            fn as_contract_name(&self, name: &Expression) -> Option<String> {
                self.$to.as_contract_name(name)
            }

            fn is_class(&self, name: &str) -> bool {
                self.$to.is_class(name)
            }

            fn current_contract(&self) -> &ContractData {
                self.$to.current_contract()
            }
        }
    };
}

delegate_contract_info!(LocalContext<'_>, contract);
delegate_storage_info!(LocalContext<'_>, contract);
delegate_events_register!(LocalContext<'_>, contract);
delegate_external_calls_register!(LocalContext<'_>, contract);

#[cfg(test)]
pub fn with_context<F: Fn(&mut LocalContext) -> ()>(f: F) {
    let data = ContractData::empty("test");
    let ctx = GlobalContext::default();
    let ctx = ContractContext::new(&ctx, data);
    let mut ctx = LocalContext::new(ctx);

    f(&mut ctx)
}

#[allow(unused_variables)]
#[cfg(test)]
pub mod test {
    use crate::model::{ir::Expression, ContractData};

    use super::{
        ContractInfo, EventsRegister, ExternalCallsRegister, FnContext, StorageInfo, TypeInfo,
    };

    #[derive(Debug)]
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

        fn find_fn(&self, class: &str, name: &str) -> Option<crate::model::ir::Function> {
            None
        }
    }

    impl ContractInfo for EmptyContext {
        fn as_contract_name(&self, name: &Expression) -> Option<String> {
            None
        }

        fn is_class(&self, name: &str) -> bool {
            false
        }

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

        fn register_local_var<T: ToString>(&mut self, name: &T, ty: &crate::model::ir::Type) {}

        fn get_local_var_by_name(&self, name: &str) -> Option<&crate::model::ir::Var> {
            todo!()
        }

        fn push_expected_type(&mut self, ty: Option<crate::model::ir::Type>) -> bool {
            false
        }

        fn drop_expected_type(&mut self) {
            todo!()
        }

        fn expected_type(&self) -> Option<&crate::model::ir::Type> {
            todo!()
        }
    }
}
