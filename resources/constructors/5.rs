#[odra::module] 
pub struct E { 
	__stack: PathStack, 
    name: odra::Variable<String>,
	text: odra::Variable<String>
} 

#[odra::module] 
impl E { 
	const PATH: &'static [ClassName; 3usize] = &[ClassName::X, ClassName::Y, ClassName::E];

    fn _x_init(&mut self) {
        {
            self.name.set(String::from("name"));
        }
    }

    fn _y_init(&mut self) {
        {
            self.text.set(String::from("text"));
        }
    } 

    #[odra(init)]
    pub fn init(&mut self) {
        {
            self._x_init();
            self._y_init();
        }
    }
}