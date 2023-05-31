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

use ink;
use ink::prelude::{string::String, vec::Vec};
use crate::CurrencyId;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RequestType {
    Create,
    Mint,
    Burn,
    Transfer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OriginType {
    Caller,
    Address,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PalletAssetRequest {
    pub origin_type: OriginType,
    pub currency_id: CurrencyId,
    pub target_address: [u8; 32],
    pub amount: u128,
}

impl PalletAssetRequest {
    fn new(
        origin_type: OriginType,
        currency_id: CurrencyId,
        target_address: [u8; 32],
        amount: u128,
    ) -> PalletAssetRequest {
        PalletAssetRequest {
            origin_type,
            currency_id,
            target_address,
            amount,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PalletAssetBalanceRequest {
    pub currency_id: CurrencyId,
    pub address: [u8; 32],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PalletAssetErr {
    /// Some error occurred.
    Other,
    /// Failed to lookup some data.
    CannotLookup,
    /// A bad origin.
    BadOrigin,
    /// A custom error in a module.
    Module,
    /// At least one consumer is remaining so the account cannot be destroyed.
    ConsumerRemaining,
    /// There are no providers so the account cannot be created.
    NoProviders,
    /// There are too many consumers so the account cannot be created.
    TooManyConsumers,
    /// An error to do with tokens.
    Token(PalletAssetTokenErr),
    /// An arithmetic error.
    Arithmetic(PalletAssetArithmeticErr),
    // unknown error
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PalletAssetArithmeticErr {
    /// Underflow.
    Underflow,
    /// Overflow.
    Overflow,
    /// Division by zero.
    DivisionByZero,
    // unknown error
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PalletAssetTokenErr {
    /// Funds are unavailable.
    NoFunds,
    /// Account that must exist would die.
    WouldDie,
    /// Account cannot exist with the funds that would be given.
    BelowMinimum,
    /// Account cannot be created.
    CannotCreate,
    /// The asset in question is unknown.
    UnknownAsset,
    /// Funds exist but are frozen.
    Frozen,
    /// Operation is not supported by the asset.
    Unsupported,
    // unknown error
    Unknown,
}

impl From<u8> for OriginType {
    fn from(origin: u8) -> OriginType {
        if origin == 0 {
            OriginType::Caller
        } else {
            OriginType::Address
        }
    }
}

impl From<PalletAssetErr> for PSP22Error {
    fn from(e: PalletAssetErr) -> PSP22Error {
        match e {
            PalletAssetErr::Other => PSP22Error::Custom(String::from("psp22 error")),
            PalletAssetErr::CannotLookup => PSP22Error::Custom(String::from("CannotLookup")),
            PalletAssetErr::BadOrigin => PSP22Error::Custom(String::from("BadOrigin")),
            PalletAssetErr::Module => PSP22Error::Custom(String::from("Module")),
            PalletAssetErr::ConsumerRemaining => {
                PSP22Error::Custom(String::from("ConsumerRemaining"))
            }
            PalletAssetErr::NoProviders => PSP22Error::Custom(String::from("NoProviders")),
            PalletAssetErr::TooManyConsumers => {
                PSP22Error::Custom(String::from("TooManyConsumers"))
            }
            PalletAssetErr::Token(_token_err) => PSP22Error::Custom(String::from("Token")),
            PalletAssetErr::Arithmetic(_arithmetic_error) => {
                PSP22Error::Custom(String::from("Arithmetic"))
            }
            _ => PSP22Error::Custom(String::from("Unnown")),
        }
    }
}

impl ink::env::chain_extension::FromStatusCode for PalletAssetErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for PalletAssetErr {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
