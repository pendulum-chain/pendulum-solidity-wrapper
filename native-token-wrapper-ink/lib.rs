#![cfg_attr(not(feature = "std"), no_std)]

use ethnum::U256;
use ink::prelude::vec::Vec;
use ink::prelude::string::String;
use runtime_common::chain_ext::{
    self, Address, Amount, ChainExtensionError, CurrencyId, OriginType,
};
use scale::Encode;

#[ink::contract]
mod native_token_wrapper {
    use crate::*;

    #[ink(storage)]
    #[derive(Default)]
    pub struct NativeTokenWrapper {
        pub origin_type: u8,
        pub currency_id: Vec<u8>,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
    }

    impl NativeTokenWrapper {
        #[ink(constructor)]
        pub fn new(
            origin_type: OriginType,
            currency_id: CurrencyId,
            name: String,
            symbol: String,
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
        pub fn name(&self) -> String {
            self.name.clone()
        }
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.decimals.clone()
        }
        #[ink(message)]
        pub fn total_supply(&self) -> u128 {
            ::ink::env::chain_extension::ChainExtensionMethod::build(1107u32)
                .input::<CurrencyId>()
                .output::<u128, false>()
                .handle_error_code::<ChainExtError>()
                .call(&self._get_currency_id())
                .unwrap()
        }
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u128 {
            let input = (self._get_currency_id(), *account.as_ref());
            let balance = ::ink::env::chain_extension::ChainExtensionMethod::build(1106u32)
                .input::<(CurrencyId, Address)>()
                .output::<u128, false>()
                .handle_error_code::<ChainExtError>()
                .call(&input)
                .unwrap();
            balance
            
        }
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u128) -> bool {
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
            true
        }
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u128) -> bool {
            let input = (
                *from.as_ref(),
                self.origin_type.into(),
                self._get_currency_id(),
                *to.as_ref(),
                amount,
            );
            ::ink::env::chain_extension::ChainExtensionMethod::build(1109u32)
                .input::<(Address, OriginType, CurrencyId, Address, Amount)>()
                .output::<Result<(), ChainExtError>, false>()
                .handle_error_code::<ChainExtError>()
                .call(&input)
                .unwrap()
                .expect("should transfer from");
            true
        }
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: u128) -> bool {
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
            true
        }
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            let input = (self._get_currency_id(), *owner.as_ref(), *spender.as_ref());
            let allowance = ::ink::env::chain_extension::ChainExtensionMethod::build(1110u32)
                .input::<(CurrencyId, Address, Address)>()
                .output::<u128, false>()
                .handle_error_code::<ChainExtError>()
                .call(&input)
                .unwrap();
            allowance
        }


        #[ink(message, selector = 0x06fdde03)]
        pub fn solidity_name(&self) -> Vec<u8> {
            self.name().encode()
        }
        #[ink(message, selector = 0x95d89b41)]
        pub fn solidity_symbol(&self) -> Vec<u8> {
            self.symbol().encode()
        }
        #[ink(message, selector = 0x313ce567)]
        pub fn solidity_decimals(&self) -> Vec<u8> {
            self.decimals().encode()
        }
        #[ink(message, selector = 0x18160ddd)]
        pub fn solidity_total_supply(&self) -> Vec<u8> {
            let total_supply = self.total_supply();
            let total_supply = U256::try_from(total_supply).unwrap().0;
            total_supply.encode()
        }
        #[ink(message, selector = 0x70a08231)]
        pub fn solidity_balance_of(&self, account: AccountId) -> Vec<u8> {
            let balance = self.balance_of(account);
            let balance = U256::try_from(balance).unwrap().0;
            balance.encode()
        }
        #[ink(message, selector = 0xa9059cbb)]
        pub fn solidity_transfer(&mut self, to: AccountId, amount: [u128; 2]) -> Vec<u8> {
            let amount: u128 = U256(amount).try_into().unwrap();
            self.transfer(to, amount).encode()
        }
        #[ink(message, selector = 0x23b872dd)]
        pub fn solidity_transfer_from(&mut self, from: AccountId, to: AccountId, amount: [u128; 2]) -> Vec<u8> {
            let amount: u128 = U256(amount).try_into().unwrap();
            self.transfer_from(from, to, amount).encode()
        }
        #[ink(message, selector = 0x095ea7b3)]
        pub fn solidity_approve(&mut self, spender: AccountId, amount: [u128; 2]) -> Vec<u8> {
            let amount: u128 = U256(amount).try_into().unwrap();
            self.approve(spender, amount).encode()
        }
        #[ink(message, selector = 0xdd62ed3e)]
        pub fn solidity_allowance(&self, owner: AccountId, spender: AccountId) -> Vec<u8> {
            let allowance = self.allowance(owner, spender);
            let allowance = U256::try_from(allowance).unwrap().0;
            allowance.encode()
        }
    }

    impl NativeTokenWrapper {
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
            let contract = NativeTokenWrapper::new(
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
