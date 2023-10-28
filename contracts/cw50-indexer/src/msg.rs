use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use crate::state::Config;

#[cw_serde]
pub struct PriceQuery {
    price: Option<Coin>
}

impl From<Config> for PriceQuery {
    fn from(value: Config) -> Self {
        Self { price: value.price }
    }
}

#[cw_serde]
pub struct InstantiateMsg {
    pub price: Option<Coin>,
    pub admin: Option<Vec<String>>,
    pub owner_can_update: bool, 
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { address: String, name: String },
    Update { address: String, to: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PriceQuery)]
    Price { },
    #[returns(cw4_group::msg::LookUpResponse)]
    LookUp { name: String },
    #[returns(cw4_group::msg::ReverseLookUpResponse)]
    ReverseLookUp { addr: String },
}
