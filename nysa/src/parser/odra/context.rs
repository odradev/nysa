use crate::model::ir::{FnImplementations, NysaExpression, NysaVar};

#[derive(Debug, Default)]
pub struct Context<'a> {
    current_fn: Option<FnImplementations>,
    storage: &'a [NysaVar],
    classes: Vec<String>,
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
}
