#[odra::module] 
pub struct E { 
	__stack: PathStack, 
    name: odra::Variable<String>,
	text: odra::Variable<String>
} 

#[odra::module] 
impl E { 
	const PATH: &'static [ClassName; 4usize] = &[ClassName::X, ClassName::Z, ClassName::Y, ClassName::E];

    fn _x_init(&mut self, _name: String) {
        {
            self.name.set(_name);
        }
    }

    fn _z_init(&mut self) {
        {
            
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
            self._x_init(String::from("X was called"));
            self._z_init();
            self._y_init(String::from("Y was called"));
        }
    }
}