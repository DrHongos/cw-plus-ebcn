use cw_storage_plus::{Map, Item};
use cosmwasm_std::{Addr, Coin};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {
    pub purchase_price: Option<Coin>
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const NAMES_KEY: &str = "names";
pub const NAMES_RESOLVER: Map<&Addr, String> = Map::new(
    NAMES_KEY,
);

pub const ADDR_KEY: &str = "address";

pub const ADDR_RESOLVER: Map<String, String> = Map::new(
    ADDR_KEY,
);