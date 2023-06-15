#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;
use scale::Encode;
use primitive_types::U256;

#[ink::contract]
mod my_psp22_pallet_asset {
    use crate::*;

    use ink::env::debug_println;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_address(&self) -> [u8; 32] {
            let caller = self.env().caller();
            *caller.as_ref()
        }

        #[ink(message, selector = 0x06fdde03)]
        pub fn name(&self) -> Vec<u8> {
            b"TestName".to_vec()
        }

        #[ink(message, selector = 0x18160ddd)]
        pub fn total_supply(&self) -> U256 {
            debug_println!("in total_supply"); // Using debug_println! with values fails apparently?
            let total_supply = U256::from(100u128);
            debug_println!("value: {}", total_supply.0[0]);
            total_supply
        }
    }
}
