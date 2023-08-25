#[odra::module] 
pub struct C { 
	__stack: PathStack, 
    name: odra::Variable<String>,
	text: odra::Variable<String>
} 

#[odra::module] 
impl C { 
	const PATH: &'static [ClassName; 3usize] = &[ClassName::X, ClassName::Y, ClassName::C];

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
    pub fn init(&mut self, _name: String, _text: String) {
        {
            self._x_init(_name);
            self._y_init(_text);
        }
    }
}