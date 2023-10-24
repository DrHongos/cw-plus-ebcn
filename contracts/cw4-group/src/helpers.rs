use std::ops::Deref;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};
use cw4::Cw4Contract;

use crate::{msg::{ExecuteMsg, MemberNamed}, ContractError};

/// Cw4GroupContract is a wrapper around Cw4Contract that provides a lot of helpers
/// for working with cw4-group contracts.
///
/// It extends Cw4Contract to add the extra calls from cw4-group.
#[cw_serde]
pub struct Cw4GroupContract(pub Cw4Contract);

impl Deref for Cw4GroupContract {
    type Target = Cw4Contract;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Cw4GroupContract {
    pub fn new(addr: Addr) -> Self {
        Cw4GroupContract(Cw4Contract(addr))
    }

    fn encode_msg(&self, msg: ExecuteMsg) -> StdResult<CosmosMsg> {
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
            funds: vec![],
        }
        .into())
    }

    pub fn update_members(&self, remove: Vec<String>, add: Vec<MemberNamed>) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::UpdateMembers { remove, add };
        self.encode_msg(msg)
    }
}

/// Sorts the slice and verifies all member addresses are unique.
pub fn validate_unique_members(members: &mut [MemberNamed]) -> Result<(), ContractError> {
    members.sort_by(|a, b| a.addr.cmp(&b.addr));
    for (a, b) in members.iter().zip(members.iter().skip(1)) {
        if a.addr == b.addr {
            return Err(ContractError::DuplicateMember {
                member: a.addr.clone(),
            });
        }
    }

    Ok(())
}

const MIN_NAME_LENGTH: u64 = 2;
const MAX_NAME_LENGTH: u64 = 30;

fn invalid_char(c: char) -> bool {
    let is_valid =
        c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '.' || c == '-' || c == '_');
    !is_valid
}

/// validate_name returns an error if the name is invalid
/// (we require 3-64 lowercase ascii letters, numbers, or . - _)
pub fn validate_name(name: &str) -> Result<(), ContractError> {
    let length = name.len() as u64;
    if (name.len() as u64) < MIN_NAME_LENGTH {
        Err(ContractError::NameTooShort {
            length,
            min_length: MIN_NAME_LENGTH,
        })
    } else if (name.len() as u64) > MAX_NAME_LENGTH {
        Err(ContractError::NameTooLong {
            length,
            max_length: MAX_NAME_LENGTH,
        })
    } else {
        match name.find(invalid_char) {
            None => Ok(()),
            Some(bytepos_invalid_char_start) => {
                let c = name[bytepos_invalid_char_start..].chars().next().unwrap();
                Err(ContractError::InvalidCharacter { c })
            }
        }
    }
}
