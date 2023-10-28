use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
//use cw4_group::helpers::Cw4GroupContract;
//    contract::query_reverse_lookup as group_reverse_look_up,

use crate::coin_helpers::assert_sent_sufficient_coin;
use crate::error::ContractError;
use crate::helpers::Cw69Contract;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, PriceQuery};
use cw4_group::msg::{LookUpResponse, ReverseLookUpResponse};
use crate::state::{Config, CONFIG, NAMES_RESOLVER, ADDR_RESOLVER};

/* 
TODO:
admin and permits!


*/


const MIN_NAME_LENGTH: u64 = 2;
const MAX_NAME_LENGTH: u64 = 30;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config = Config {   
        purchase_price: msg.purchase_price,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { address, name } => execute_register(deps, env, info, address, name),
        ExecuteMsg::Change { address, to } => execute_change(deps, env, info, address, to),
    }
}

pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
    name: String,
) -> Result<Response, ContractError> {
    let address_p = deps.api.addr_validate(&address)?;   
    validate_name(&name)?;
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.purchase_price)?;

//    let key = name.as_bytes();
    
    if (ADDR_RESOLVER.may_load(deps.storage, name.clone())?).is_some() {
        return Err(ContractError::NameTaken { name });
    }
    if let Some(name_p) = NAMES_RESOLVER.may_load(deps.storage, &address_p)? {
        return Err(ContractError::AddressAlreadySet { address, name: (name_p) });
    }

    NAMES_RESOLVER.save(deps.storage,  &address_p, &name)?;
    ADDR_RESOLVER.save(deps.storage, name, &address)?;
    Ok(Response::default())
}

pub fn execute_change(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,     // when added admin limits
    address: String,
    to: String,
) -> Result<Response, ContractError> {
    //let config = CONFIG.load(deps.storage)?;
    let address_p = deps.api.addr_validate(&address)?;   
    validate_name(&to)?;
    if (ADDR_RESOLVER.may_load(deps.storage, to.clone())?).is_some() {
        return Err(ContractError::NameTaken { name: to });
    }
    if let Some(name_actual) = ADDR_RESOLVER.may_load(deps.storage, address.clone())? {    
        NAMES_RESOLVER.remove(deps.storage, &address_p);
        ADDR_RESOLVER.remove(deps.storage, name_actual);
        NAMES_RESOLVER.save(deps.storage, &address_p, &to)?;
        ADDR_RESOLVER.save(deps.storage, to, &address)?;
        
        Ok(Response::default())
    } else {
        Err(ContractError::Unauthorized {  })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::LookUp { name } => to_binary(&query_lookup(deps, name)?),
        QueryMsg::ReverseLookUp { addr } => to_binary(&query_reverse_lookup(deps, addr)?),
        QueryMsg::Price { } => to_binary::<PriceQuery>(&CONFIG.load(deps.storage)?.into()),
    }
}

// TODO: add the check for points (indexing) and query contracts
pub fn query_reverse_lookup(deps: Deps, addr: String) -> StdResult<ReverseLookUpResponse> {
    let addr = deps.api.addr_validate(&addr)?;
    let name = NAMES_RESOLVER.may_load(deps.storage, &addr)?;
    Ok(ReverseLookUpResponse { name })
}

pub fn query_lookup(deps: Deps, name: String) -> StdResult<LookUpResponse> {
    let mut links = name.split('.').rev().into_iter();
    let links_s = links.clone().count();
    let fname = links.next();
    let faddr = ADDR_RESOLVER.may_load(deps.storage, fname.unwrap().to_string())?;
    match links_s {
        0 => { Ok(LookUpResponse { addr: None })}
        1 => {
            Ok(LookUpResponse { addr: faddr })        
        },
        _ => {
            let mut addr_a = faddr;
            while let Some(name) = links.next() {
                let addr_p = deps.api.addr_validate(&addr_a.unwrap())?;
                let pname = Cw69Contract(addr_p)
                    .look_up(&deps.querier, name.into())?;
                addr_a = pname;
            } 
            Ok(LookUpResponse { addr: addr_a })
        }
    }
}

fn invalid_char(c: char) -> bool {
    let is_valid =
        c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '.' || c == '-' || c == '_');
    !is_valid
}

fn validate_name(name: &str) -> Result<(), ContractError> {
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
