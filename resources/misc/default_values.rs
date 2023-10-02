{{DEFAULT_MODULES}}
pub mod my_contract {
    #![allow(unused_braces, unused_mut, unused_parens, non_snake_case, unused_imports)]

    {{DEFAULT_IMPORTS}}

    {{STACK_DEF}}

    #[derive(Clone)]
    enum ClassName {
        MyContract
    }

    #[odra::module] 
    pub struct MyContract { 
        __stack: PathStack, 
        my_number: odra::Variable<nysa_types::U256>,
        min_int: odra::Variable<i16>,
        boo: odra::Variable<bool>,
        my_uint: odra::Variable<nysa_types::U192>,
        my_uint_2: odra::Variable<nysa_types::U256>,
    } 

    #[odra::module] 
    impl MyContract { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::MyContract];

        pub fn get_my_number(&self) -> nysa_types::U256 {
            self.__stack.push_path_on_stack(Self::PATH);
            let result = self.super_get_my_number();
            self.__stack.drop_one_from_stack();
            result
        }

        fn super_get_my_number(&self) -> nysa_types::U256 {
            let __class = self.__stack.pop_from_top_path();
            match __class {
                ClassName::MyContract => {
                    return self.my_number.get_or_default();
                }
                #[allow(unreachable_patterns)]
                _ => self.super_get_my_number(),
            }
        }

        #[odra(init)]
        pub fn init(&mut self, _my_uint: nysa_types::U256) {
            self.my_number.set(nysa_types::U256::from_limbs_slice(&[42u64]));
            self.min_int.set(i16::MIN);
            self.boo.set(true);
            self.my_uint.set(nysa_types::U192::from_limbs_slice(&[123u64]));
            self.my_uint_2.set(_my_uint);
        }
    }
}