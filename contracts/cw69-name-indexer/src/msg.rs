use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use crate::state::Config;

#[cw_serde]
pub struct PriceQuery {
    price: Option<Coin>
}

impl From<Config> for PriceQuery {
    fn from(value: Config) -> Self {
        Self { price: value.purchase_price }
    }
}

#[cw_serde]
pub struct InstantiateMsg {
    pub purchase_price: Option<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { address: String, name: String },
    Change { address: String, to: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PriceQuery)]
    Price { },
    #[returns(cw4_group::msg::LookUpResponse)]
    LookUp { addr: String },
    #[returns(cw4_group::msg::ReverseLookUpResponse)]
    ReverseLookUp { name: String },
}
