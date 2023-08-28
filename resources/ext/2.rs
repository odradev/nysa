#[odra::module] 
pub struct Caller { 
	__stack: PathStack, 
} 

#[odra::module] 
impl Caller { 
	const PATH: &'static [ClassName; 1usize] = &[ClassName::Caller];

    #[odra(init)]
    pub fn init(&mut self) {
        {}
    }

    pub fn set_x(&mut self, _callee: Option<odra::types::Address>, _x: odra::types::U256) {
        self.__stack.push_path_on_stack(Self::PATH);
        let result = self.super_set_x(_callee, _x);
        self.__stack.drop_one_from_stack();
        result
    }

    fn super_set_x(&mut self, _callee: Option<odra::types::Address>, _x: odra::types::U256) {
        let __class = self.__stack.pop_from_top_path();
        match __class {
            ClassName::Caller => {
                let _callee = CalleeRef::at(odra::UnwrapOrRevert::unwrap_or_revert(_callee));
                let x = _callee.set_x(_x);
            }
            #[allow(unreachable_patterns)]
            _ => self.super_set_x(_callee, _x)
        }
    }

    pub fn set_x_from_address(&mut self, _addr: Option<odra::types::Address>, _x: odra::types::U256) {
        self.__stack.push_path_on_stack(Self::PATH);
        let result = self.super_set_x_from_address(_addr, _x);
        self.__stack.drop_one_from_stack();
        result
    }

    fn super_set_x_from_address(&mut self, _addr: Option<odra::types::Address>, _x: odra::types::U256) {
        let __class = self.__stack.pop_from_top_path();
        match __class {
            ClassName::Caller => {
                let callee = CalleeRef::at(odra::UnwrapOrRevert::unwrap_or_revert(_addr));
                callee.set_x(_x);
            }
            #[allow(unreachable_patterns)]
            _ => self.super_set_x_from_address(_addr, _x)
        }
    }  
}