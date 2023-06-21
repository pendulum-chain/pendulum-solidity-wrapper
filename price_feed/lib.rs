#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod price_feed {
    use ink::prelude::vec::Vec;
    use runtime_common::chain_ext::{CoinInfo, Blockchain, Symbol, ChainExtensionError};

    #[ink(storage)]
    pub struct PriceFeed {}
    impl PriceFeed {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }
        #[ink(message)]
        pub fn get_coin_info(
            &mut self,
            blockchain: Blockchain,
            symbol: Symbol,
        ) -> Result<CoinInfo, ChainExtError> {
            let input = (blockchain, symbol);
            ::ink::env::chain_extension::ChainExtensionMethod::build(1200u32)
                .input::<(Blockchain, Symbol)>()
                .output::<Result<CoinInfo, ChainExtError>, false>()
                .handle_error_code::<ChainExtError>()
                .call(&input)?
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ChainExtError(ChainExtensionError);
    impl ink::env::chain_extension::FromStatusCode for ChainExtError {
        fn from_status_code(status_code: u32) -> Result<(), Self> {
            match status_code {
                0 => Ok(()),
                _ => Err(Self(ChainExtensionError::Unknown)),
            }
        }
    }

}
