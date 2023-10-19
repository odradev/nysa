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
        min_int: odra::Variable<nysa_types::I256>,
        neg: odra::Variable<nysa_types::I32>,
        boo: odra::Variable<bool>,
        my_uint_2: odra::Variable<nysa_types::U256>,
    } 

    #[odra::module] 
    impl MyContract { 
        const PATH: &'static [ClassName; 1usize] = &[ClassName::MyContract];
        pub const MY_UINT: nysa_types::U192 = nysa_types::U192::from_limbs([123u64, 0u64, 0u64]);
        pub const NAME: &str = "my name";
        pub const FLAG: bool = false;
        pub const BYTE_ARRAY: nysa_types::FixedBytes<2usize> = nysa_types::FixedBytes([171u8, 205u8]);
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
            self.min_int.set(nysa_types::I256::MIN);
            self.neg.set(-nysa_types::I32::from_limbs_slice(&[9u64]));
            self.boo.set(true);
            self.my_uint_2.set(_my_uint);
        }
    }
}