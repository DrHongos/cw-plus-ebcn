use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, CustomQuery, QuerierWrapper, QueryRequest, StdResult, WasmQuery,
//    WasmMsg, CosmosMsg, 
};
use cw4_group::msg::LookUpResponse;
use cw5::ReverseLookUpResponse;
use crate::msg::QueryMsg;

#[cw_serde]
pub struct Cw50Contract(pub Addr);

impl Cw50Contract {                         // add cw5 as interface?
    pub fn new(addr: Addr) -> Self {
        Cw50Contract(addr)
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
    pub fn reverse_look_up(&self, querier: &QuerierWrapper, addr: String) -> StdResult<Option<String>> {
        let query = self.encode_smart_query(QueryMsg::ReverseLookUp { addr })?;
        let res: ReverseLookUpResponse = querier.query(&query)?;
        Ok(res.name)
    }


}