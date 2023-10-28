use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum Cw5ExecuteMsg {
    Register { addr: String, name: String },
    Update { addr: String, name: String },      // should be addr, key, value
}
