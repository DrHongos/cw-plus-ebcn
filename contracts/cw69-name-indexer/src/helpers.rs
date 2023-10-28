use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, CustomQuery, QuerierWrapper, QueryRequest, StdResult, WasmQuery,
//    WasmMsg, CosmosMsg, 
};
use cw4_group::msg::LookUpResponse;
use crate::msg::QueryMsg;

#[cw_serde]
pub struct Cw69Contract(pub Addr);

impl Cw69Contract {
    pub fn new(addr: Addr) -> Self {
        Cw69Contract(addr)
    }

    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    fn encode_smart_query<Q: CustomQuery>(&self, msg: QueryMsg) -> StdResult<QueryRequest<Q>> {
        Ok(WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into())
    }
    /* 
    // example (from cw4)
    pub fn admin(&self, querier: &QuerierWrapper) -> StdResult<Option<String>> {
        let query = self.encode_smart_query(Cw4QueryMsg::Admin {})?;
        let res: AdminResponse = querier.query(&query)?;
        Ok(res.admin)
    }
     */
    // lookUp

    pub fn look_up(&self, querier: &QuerierWrapper, name: String) -> StdResult<Option<String>> {
        let query = self.encode_smart_query(QueryMsg::LookUp { name })?;
        let res: LookUpResponse = querier.query(&query)?;
        Ok(res.addr)
    }

    // should include the others (that i dont indend to call?)

}