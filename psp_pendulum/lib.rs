#![cfg_attr(not(feature = "std"), no_std)]

use ethnum::U256;
use ink::prelude::vec::Vec;
use runtime_common::chain_ext::{
    self, Address, Amount, ChainExtensionError, CurrencyId, OriginType,
};
use scale::Encode;

#[ink::contract]
mod my_psp22_pallet_asset {
    use crate::*;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MyPSP22 {
        pub origin_type: u8,
        pub currency_id: Vec<u8>,
        pub name: Vec<u8>,
        pub symbol: Vec<u8>,
        pub decimals: u8,
    }

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(
            origin_type: OriginType,
            currency_id: CurrencyId,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> Self {
            let origin_type = if origin_type == OriginType::Caller {
                0
            } else {
                1
            };
            Self {
                origin_type,
                currency_id: currency_id.encode(),
                name,
                symbol,
                decimals,
            }
        }

        #[ink(message)]
        pub fn get_address(&self) -> [u8; 32] {
            let caller = self.env().caller();
            *caller.as_ref()
        }

        #[ink(message, selector = 0x06fdde03)]
        pub fn name(&self) -> Vec<u8> {
            self.name.clone()
        }

        #[ink(message, selector = 0x95d89b41)]
        pub fn symbol(&self) -> Vec<u8> {
            self.symbol.clone()
        }

        #[ink(message, selector = 0x313ce567)]
        pub fn decimals(&self) -> u8 {
            self.decimals.clone()
        }

        #[ink(message, selector = 0x18160ddd)]
        pub fn total_supply(&self) -> [u128; 2] {
            let total_supply = ::ink::env::chain_extension::ChainExtensionMethod::build(1107u32)
                .input::<CurrencyId>()
                .output::<u128, false>()
                .handle_error_code::<ChainExtError>()
                .call(&self._get_currency_id())
                .unwrap();
            U256::try_from(total_supply).unwrap().0
        }

        #[ink(message, selector = 0x70a08231)]
        pub fn balance_of(&self, account: AccountId) -> [u128; 2] {
            let input = (self._get_currency_id(), *account.as_ref());
            let balance = ::ink::env::chain_extension::ChainExtensionMethod::build(1106u32)
                .input::<(CurrencyId, Address)>()
                .output::<u128, false>()
                .handle_error_code::<ChainExtError>()
                .call(&input)
                .unwrap();
            U256::try_from(balance).unwrap().0
        }

        #[ink(message, selector = 0xa9059cbb)]
        pub fn transfer(&mut self, to: AccountId, amount: [u128; 2]) {
            let amount: u128 = U256(amount).try_into().unwrap();
            let input = (
                self.origin_type.into(),
                self._get_currency_id(),
                *to.as_ref(),
                amount.into(),
            );
            ::ink::env::chain_extension::ChainExtensionMethod::build(1105u32)
                .input::<(OriginType, CurrencyId, Address, Amount)>()
                .output::<Result<(), ChainExtError>, false>()
                .handle_error_code::<ChainExtError>()
                .call(&input)
                .unwrap()
                .expect("should transfer");
        }

        #[ink(message, selector = 0x23b872dd)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: [u128; 2]) {
            let amount: u128 = U256(amount).try_into().unwrap();
            let input = (
                self.origin_type.into(),
                self._get_currency_id(),
                *to.as_ref(),
                amount,
            );
            ::ink::env::chain_extension::ChainExtensionMethod::build(1109u32)
                .input::<([u8; 32], (OriginType, CurrencyId, Address, Amount))>()
                .output::<Result<(), ChainExtError>, false>()
                .handle_error_code::<ChainExtError>()
                .call(&(*from.as_ref(), input))
                .unwrap()
                .expect("should transfer from");
        }

        #[ink(message, selector = 0x095ea7b3)]
        pub fn approve(&mut self, spender: AccountId, amount: Balance) {
            let input = (
                self.origin_type.into(),
                self._get_currency_id(),
                *spender.as_ref(),
                amount.into(),
            );
            ::ink::env::chain_extension::ChainExtensionMethod::build(1108u32)
                .input::<(OriginType, CurrencyId, Address, Amount)>()
                .output::<Result<(), ChainExtError>, false>()
                .handle_error_code::<ChainExtError>()
                .call(&input)
                .unwrap()
                .unwrap();
        }

        #[ink(message, selector = 0xdd62ed3e)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> [u128; 2] {
            let input = (self._get_currency_id(), *owner.as_ref(), *spender.as_ref());
            let allowance = ::ink::env::chain_extension::ChainExtensionMethod::build(1110u32)
                .input::<(CurrencyId, Address, Address)>()
                .output::<u128, false>()
                .handle_error_code::<ChainExtError>()
                .call(&input)
                .unwrap();
            U256::try_from(allowance).unwrap().0
        }
    }

    impl MyPSP22 {
        fn _get_currency_id(&self) -> CurrencyId {
            chain_ext::decode(self.currency_id.clone()).unwrap()
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn encode_decode_works() {
            let contract = MyPSP22::new(
                0.into(),
                CurrencyId::XCM(2),
                "name".into(),
                "symbol".into(),
                0,
            );
            assert_eq!(contract._get_currency_id(), CurrencyId::XCM(2));
        }
    }
}
