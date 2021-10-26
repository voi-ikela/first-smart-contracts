#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
        my_value: ink_storage::collections::HashMap<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                my_value: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
                my_value: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value = self.value + by
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.env().caller();
            self.my_number_or_zero(&caller)
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let caller = self.env().caller();
            self.my_value
                .entry(caller)
                .and_modify(|old_value| *old_value += by)
                .or_insert(by);
        }

        #[ink(message)]
        pub fn my_number_or_zero(&self, of: &AccountId) -> i32 {
            let value = self.my_value.get(of).unwrap_or(&0);
            *value
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut incrementer = Incrementer::new(42);
            assert_eq!(incrementer.get(), 42);
            incrementer.inc(5);
            assert_eq!(incrementer.get(), 47);
            incrementer.inc(-50);
            assert_eq!(incrementer.get(), -3);
        }

        #[ink::test]
        fn my_value_works() {
            let mut incrementer = Incrementer::new(11);
            assert_eq!(incrementer.get(), 11);
            assert_eq!(incrementer.get_mine(), 0);

            incrementer.inc_mine(5);
            assert_eq!(incrementer.get_mine(), 5);
            incrementer.inc_mine(10);
            assert_eq!(incrementer.get_mine(), 15);
        }
    }
}
