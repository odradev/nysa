#![allow(unused_braces, non_snake_case)]
impl odra::types::contract_def::Node for PathStack {
    const COUNT: u32 = 0;
    const IS_LEAF: bool = false;
}
impl odra::OdraItem for PathStack {
    fn is_module() -> bool {
        false
    }
}
impl odra::StaticInstance for PathStack {
    fn instance<'a>(keys: &'a [&'a str]) -> (Self, &'a [&'a str]) {
        (PathStack::default(), keys)
    }
}
impl odra::DynamicInstance for PathStack {
    #[allow(unused_variables)]
    fn instance(namespace: &[u8]) -> Self {
        PathStack::default()
    }
}
#[derive(Clone)]
struct PathStack {
    stack: std::sync::Arc<std::sync::Mutex<Vec<Vec<ClassName>>>>,
}
impl PathStack {
    pub fn new() -> Self {
        PathStack {
            stack: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
    pub fn push_path_on_stack(&self, path: &[ClassName]) {
        let mut stack = self.stack.lock().unwrap();
        stack.push(path.to_vec());
    }
    pub fn drop_one_from_stack(&self) {
        let mut stack = self.stack.lock().unwrap();
        stack.pop().unwrap();
    }
    pub fn pop_from_top_path(&self) -> ClassName {
        let mut stack = self.stack.lock().unwrap();
        let mut path = stack.pop().unwrap();
        let class = path.pop().unwrap();
        stack.push(path);
        class
    }
}
impl Default for PathStack {
    fn default() -> PathStack {
        PathStack::new()
    }
}
#[derive(Clone)]
enum ClassName {
    D, Y, X
}
#[odra::module] 
pub struct D { 
	__stack: PathStack, 
    name: odra::Variable<String>,
	text: odra::Variable<String>
} 

#[odra::module] 
impl D { 
	const PATH: &'static [ClassName; 3usize] = &[ClassName::X, ClassName::Y, ClassName::D];

    fn _x_init(&mut self, _name: String) {
        {
            self.name.set(_name);
        }
    }

    fn _y_init(&mut self, _text: String) {
        {
            self.text.set(_text);
        }
    } 

    #[odra(init)]
    pub fn init(&mut self) {
        {
            self._x_init(String::from("Input to XXX"));
            self._y_init(String::from("Input to Y"));
        }
    }
}