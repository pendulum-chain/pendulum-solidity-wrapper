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
        pub fn get_coin_info(&mut self, blockchain: [u8; 32], symbol: [u8; 32]) -> Result<CoinInfo, DispatchError> {      
            let input = (blockchain, symbol);
            ::ink::env::chain_extension::ChainExtensionMethod::build(7777u32)
            .input::<([u8; 32], [u8; 32])>()
            .output::<CoinInfo, false>()
            .handle_error_code::<DispatchError>()
            .call(&input)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct CoinInfo {
        pub symbol: Vec<u8>,
        pub name: Vec<u8>,
        pub blockchain: Vec<u8>,
        pub supply: u128,
        pub last_update_timestamp: u64,
        pub price: u128,
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
            assert_eq!(price_feed.get_coin_info([0u8; 32], [0u8; 32]), Ok(CoinInfo { symbol: vec![], name: vec![], blockchain: vec![], supply: 0, last_update_timestamp: 0, price: 0 }));
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = AaaRef::default();

            // When
            let contract_account_id = client
                .instantiate("aaa", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<AaaRef>(contract_account_id.clone())
                .call(|aaa| aaa.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = AaaRef::new(false);
            let contract_account_id = client
                .instantiate("aaa", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<AaaRef>(contract_account_id.clone())
                .call(|aaa| aaa.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<AaaRef>(contract_account_id.clone())
                .call(|aaa| aaa.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<AaaRef>(contract_account_id.clone())
                .call(|aaa| aaa.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}