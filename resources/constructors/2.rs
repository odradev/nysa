#[odra::module] 
pub struct B { 
	__stack: PathStack, 
    name: odra::Variable<String>,
	text: odra::Variable<String>
} 

#[odra::module] 
impl B { 
	const PATH: &'static [ClassName; 3usize] = &[ClassName::X, ClassName::Y, ClassName::B];

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
            self._x_init(String::from("Input to X"));
            self._y_init(String::from("Input to Y"));
        }
    }
}