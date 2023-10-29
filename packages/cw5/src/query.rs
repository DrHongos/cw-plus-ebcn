use cosmwasm_schema::cw_serde;
//use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[cw_serde]
pub enum Cw5QueryMsg {
    LookUp { name: String },
    ReverseLookUp { address: String },
    //Metadata { address: String, key: String },
}

pub const NAMES_KEY: &str = "names";
pub const NAMES_RESOLVER: Map<String, String> = Map::new( // &Addr, String
    NAMES_KEY,
);

pub const ADDR_KEY: &str = "address";
pub const ADDR_RESOLVER: Map<String, String> = Map::new(
    ADDR_KEY,
);
