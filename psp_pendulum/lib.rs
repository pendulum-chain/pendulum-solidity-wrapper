// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
#![cfg_attr(not(feature = "std"), no_std)]

use ethnum::U256;
use ink::{prelude::vec::Vec, storage::traits::StorageLayout};
use scale::{Decode, Encode, Codec, Input};
mod psp_pendulum_lib;

use crate::psp_pendulum_lib::{PalletAssetErr, PSP22Error};
use runtime_common::chain_ext::{self, ChainExtensionError, CurrencyId, Address, Amount, OriginType};

#[ink::contract]
mod my_psp22_pallet_asset {
    use crate::*;
    use ink::codegen::StaticEnv;

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
            let b = self._total_supply();
            let total_supply_u256: U256 = U256::try_from(b).unwrap();
            total_supply_u256.0
        }

        #[ink(message, selector = 0x70a08231)]
        pub fn balance_of(&self, account: AccountId) -> [u128; 2] {
            let b = self._balance_of(account);
            let balance_u256: U256 = U256::try_from(b).unwrap();
            balance_u256.0
        }

        #[ink(message, selector = 0xa9059cbb)]
        pub fn transfer(&mut self, to: AccountId, amount: [u128; 2]) {
            let amount: u128 = U256(amount).try_into().unwrap();
            self._transfer(to, amount, Vec::<u8>::new())
                .expect("should transfer");
        }

        #[ink(message, selector = 0x23b872dd)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: [u128; 2]) {
            let amount: u128 = U256(amount).try_into().unwrap();
            self._transfer_from(from, to, amount, Vec::<u8>::new())
                .expect("should transfer from");
        }

        #[ink(message, selector = 0x095ea7b3)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) {
            self._approve(spender, value).unwrap();
        }

        #[ink(message, selector = 0xdd62ed3e)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> [u128; 2] {
            let b = self._allowance(owner, spender);
            let balance_u256: U256 = U256::try_from(b).unwrap();
            balance_u256.0
        }
    }

    impl MyPSP22 {
        fn _get_currency_id(&self) -> CurrencyId {
            chain_ext::decode(self.currency_id.clone()).unwrap()
        }
        fn _total_supply(&self) -> Balance {
            ::ink::env::chain_extension::ChainExtensionMethod::build(1107u32)
            .input::<CurrencyId>()
            .output::<u128, false>()
            .handle_error_code::<PalletAssetErr>()
            .call(&self._get_currency_id())
            .unwrap()
        }

        fn _balance_of(&self, owner: AccountId) -> Balance {
            let input = (self._get_currency_id(), *owner.as_ref());
            ::ink::env::chain_extension::ChainExtensionMethod::build(1106u32)
                .input::<(CurrencyId, Address)>()
                .output::<u128, false>()
                .handle_error_code::<PalletAssetErr>()
                .call(&input)
                .unwrap()
        }

        fn _transfer(
            &mut self,
            to: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let input = (self.origin_type.into(), self._get_currency_id(), *to.as_ref(), amount.into());
            let result = ::ink::env::chain_extension::ChainExtensionMethod::build(1105u32)
                .input::<(OriginType, CurrencyId, Address, Amount)>()
                .output::<Result<(), PalletAssetErr>, false>()
                .handle_error_code::<PalletAssetErr>()
                .call(&input)
                .unwrap();

            match result {
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Ok(_) => {
                    Result::<(), PSP22Error>::Ok(())
                }
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Err(e) => {
                    Result::<(), PSP22Error>::Err(PSP22Error::from(e))
                }
            }
        }

        fn _transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let input = (self.origin_type.into(), self._get_currency_id(), *to.as_ref(), amount);
            let result = ::ink::env::chain_extension::ChainExtensionMethod::build(1109u32)
                .input::<([u8; 32], (OriginType, CurrencyId, Address, Amount))>()
                .output::<Result<(), PalletAssetErr>, false>()
                .handle_error_code::<PalletAssetErr>()
                .call(&(*from.as_ref(), input))
                .unwrap();

            match result {
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Ok(_) => {
                    Result::<(), PSP22Error>::Ok(())
                }
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Err(e) => {
                    Result::<(), PSP22Error>::Err(PSP22Error::from(e))
                }
            }
        }

        fn _approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
            let input = (self.origin_type.into(), self._get_currency_id(), *spender.as_ref(), value.into());
            let result = ::ink::env::chain_extension::ChainExtensionMethod::build(1108u32)
                .input::<(OriginType, CurrencyId, Address, Amount)>()
                .output::<Result<(), PalletAssetErr>, false>()
                .handle_error_code::<PalletAssetErr>()
                .call(&input)
                .unwrap();

            match result {
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Ok(_) => {
                    Result::<(), PSP22Error>::Ok(())
                }
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Err(e) => {
                    Result::<(), PSP22Error>::Err(PSP22Error::from(e))
                }
            }
        }

        fn _allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            let input = (self._get_currency_id(), *owner.as_ref(), *spender.as_ref());
            ::ink::env::chain_extension::ChainExtensionMethod::build(1110u32)
            .input::<(CurrencyId, Address, Address)>()
            .output::<u128, false>()
            .handle_error_code::<PalletAssetErr>()
            .call(&input)
            .unwrap()
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
            let contract = MyPSP22::new(0.into(), CurrencyId::XCM(2), "name".into(), "symbol".into(), 0);
            assert_eq!(contract._get_currency_id(), CurrencyId::XCM(2));
        }
    }
}
