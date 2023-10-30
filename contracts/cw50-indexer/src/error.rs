use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},
    
    #[error("Address {address} is set to {name}")]
    AddressAlreadySet { address: String, name: String },

    #[error("Address {address} is unset")]
    AddressUnSet { address: String },
    
    #[error("Name already used")]
    NameTaken { name: String },

    #[error("Name too short (length {length} min_length {min_length})")]
    NameTooShort { length: u64, min_length: u64 },

    #[error("Name too long (length {length} min_length {max_length})")]
    NameTooLong { length: u64, max_length: u64 },

    #[error("Invalid character(char {c}")]
    InvalidCharacter { c: char },
}
