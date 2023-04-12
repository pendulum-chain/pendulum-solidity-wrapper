#![cfg_attr(not(feature = "std"), no_std)]


#[ink::contract]
mod price_feed {
    use ink::prelude::{vec::Vec};
    #[ink(storage)]
    pub struct PriceFeed {}
    impl PriceFeed {        
        #[ink(constructor)]
        pub fn new() -> Self {
            Self{}
        }
        #[ink(message)]
        pub fn get_coin_info(&mut self, blockchain: [u8; 32], symbol: [u8; 32]) -> Result<Vec<u8>, DispatchError> {      
            let input = (blockchain, symbol);
            ::ink::env::chain_extension::ChainExtensionMethod::build(7777u32)
            .input::<([u8; 32], [u8; 32])>()
            .output::<Vec<u8>, false>()
            .handle_error_code::<DispatchError>()
            .call(&input)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum DispatchError {
        /// Some error occurred.
        Other(
            #[codec(skip)]
            &'static str,
        )
    }
    impl ink::env::chain_extension::FromStatusCode for DispatchError {
        fn from_status_code(status_code: u32) -> Result<(), Self> {
            match status_code {
                0 => Ok(()),
                _ => panic!("encountered unknown status code"),
            }
        }
    }

    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut price_feed = PriceFeed::new();
            assert_eq!(price_feed.get_coin_info([0u8; 32], [0u8; 32]), Ok(vec![]));
        }
    }
}