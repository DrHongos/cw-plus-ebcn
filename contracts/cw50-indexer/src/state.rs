use cosmwasm_std::Coin;
use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub price: Option<Coin>,
    pub admin: Option<Vec<String>>,
    pub owner_can_update: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
